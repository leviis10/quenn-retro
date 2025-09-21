use crate::AppState;
use crate::handlers::health;
use axum::Router;
use axum::routing::get;
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/liveness", get(health::liveness))
        .route("/readiness", get(health::readiness))
}
