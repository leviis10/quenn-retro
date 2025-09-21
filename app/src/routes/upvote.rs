use crate::AppState;
use crate::handlers;
use axum::Router;
use axum::routing::{delete, post};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(handlers::upvote::create))
        .route("/{id}", delete(handlers::upvote::delete_by_id))
}
