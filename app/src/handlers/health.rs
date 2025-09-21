use crate::AppState;
use crate::dtos::health::HealthResponse;
use crate::errors::Result;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use std::sync::Arc;

pub async fn liveness() -> Result<(StatusCode, Json<HealthResponse>)> {
    Ok((
        StatusCode::OK,
        Json(HealthResponse {
            status: String::from("UP"),
        }),
    ))
}

pub async fn readiness(
    State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<HealthResponse>)> {
    state.db.ping().await?;
    Ok((
        StatusCode::OK,
        Json(HealthResponse {
            status: String::from("UP"),
        }),
    ))
}
