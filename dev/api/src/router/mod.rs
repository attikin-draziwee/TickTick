use axum::{Router, routing::get};

use crate::{AppState, handler, router};
mod user;

pub fn main_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(handler::health::health))
        .nest("/user", router::user::router())
        .with_state(state)
}
