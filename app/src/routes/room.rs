use crate::AppState;
use crate::handlers::room;
use axum::Router;
use axum::routing::{delete, get, post, put};
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(room::create))
        .route("/", get(room::find_all))
        .route("/{id}", get(room::get_by_id))
        .route("/{id}", put(room::update_by_id))
        .route("/{id}", delete(room::delete_by_id))
}
