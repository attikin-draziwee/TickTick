use axum::{Router, routing::get};

use crate::handler;

pub fn main_router() -> Router {
    Router::new().route("/health", get(handler::health::health))
}
