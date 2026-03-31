use axum::{Router, routing::get};

use crate::{AppState, handler};

pub fn main_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(handler::health::health))
        .with_state(state)
}
