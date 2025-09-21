use crate::{AppState, handlers};
use axum::Router;
use axum::routing::{delete, post, put};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(handlers::note::create))
        .route("/{id}", put(handlers::note::update_by_id))
        .route("/{id}", delete(handlers::note::delete_by_id))
}
