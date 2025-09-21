use crate::AppState;
use axum::Router;
use std::sync::Arc;

mod health;
mod note;
mod room;
mod upvote;

pub fn register() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/health", health::router())
        .nest("/api/v1/rooms", room::router())
        .nest("/api/v1/notes", note::router())
        .nest("/api/v1/upvotes", upvote::router())
}
