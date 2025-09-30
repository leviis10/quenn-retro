use axum::http::{HeaderValue, Method};
use sea_orm::{Database, DatabaseConnection};
use std::error::Error;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::trace::TraceLayer;
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
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let db_host = std::env::var("DB_HOST")?;
    let db_name = std::env::var("DB_NAME")?;
    let db_username = std::env::var("DB_USERNAME")?;
    let db_password = std::env::var("DB_PASSWORD")?;
    let db = Database::connect(format!(
        "postgres://{db_username}:{db_password}@{db_host}/{db_name}"
    ))
    .await?;
    tracing::info!("Connected to the database");

    let state = Arc::new(AppState { db });

    let allowed_origins: Vec<HeaderValue> = std::env::var("ALLOWED_ORIGINS")?
        .split(',')
        .map(|value| value.parse().unwrap())
        .collect();

    let app = routes::register()
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
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
                ),
        )
        .with_state(state);

    let port = std::env::var("PORT")?;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    tracing::info!("Server is listening on {port}");

    axum::serve(listener, app).await?;
    Ok(())
}
