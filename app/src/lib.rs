use crate::constants::environment_variables::database::{
    DB_HOST, DB_NAME, DB_PASSWORD, DB_USERNAME,
};
use crate::constants::environment_variables::server::{ALLOWED_ORIGINS, PORT, TIMEOUT_DURATION};
use anyhow::Context;
use axum::http::{HeaderName, HeaderValue, Method};
use sea_orm::{Database, DatabaseConnection};
use std::env;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::signal;
use tokio::time::Instant;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod constants;
mod dtos;
mod entities;
mod errors;
mod extractors;
mod handlers;
mod repositories;
mod routes;
mod services;

struct AppState {
    db: DatabaseConnection,
}

pub async fn start() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let db_host = env::var(DB_HOST).context("DB_HOST Environment variable not found")?;
    let db_name = env::var(DB_NAME).context("DB_NAME environment variable not found")?;
    let db_username =
        env::var(DB_USERNAME).context("DB_USERNAME environment variable not found")?;
    let db_password =
        env::var(DB_PASSWORD).context("DB_PASSWORD environment variable not found")?;
    let db = Database::connect(format!(
        "postgres://{db_username}:{db_password}@{db_host}/{db_name}"
    ))
    .await
    .context("Failed to connect to the database")?;
    tracing::info!("Connected to the database");

    let state = Arc::new(AppState { db });

    let allowed_origins: Vec<HeaderValue> = env::var(ALLOWED_ORIGINS)
        .unwrap_or_else(|_err| String::from(""))
        .split(',')
        .map(|value| value.parse().unwrap())
        .collect();

    let x_request_id = HeaderName::from_static("x-request-id");
    let timeout_duration: u64 = env::var(TIMEOUT_DURATION)
        .unwrap_or(String::from("30"))
        .parse()
        .context("Failed to parse TIMEOUT_DURATION environment variable into u64")?;
    let app = routes::register()
        .layer(
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::new().include_headers(true))
                        .on_response(DefaultOnResponse::new().include_headers(true)),
                )
                .layer(
                    CorsLayer::new()
                        .allow_origin(AllowOrigin::list(allowed_origins))
                        .allow_methods([
                            Method::GET,
                            Method::POST,
                            Method::PUT,
                            Method::PATCH,
                            Method::DELETE,
                        ]),
                )
                .layer(CompressionLayer::new())
                .layer(TimeoutLayer::new(Duration::from_secs(timeout_duration)))
                .layer(SetRequestIdLayer::new(
                    x_request_id.clone(),
                    MakeRequestUuid,
                ))
                .layer(PropagateRequestIdLayer::new(x_request_id)),
        )
        .with_state(state);

    let port = env::var(PORT).unwrap_or(String::from("3000"));
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    tracing::info!("started on port {port}");
    tracing::info!("Started in {:?}", start_time.elapsed());
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Graceful shutdown complete");
}
