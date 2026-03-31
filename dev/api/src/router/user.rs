use axum::{Router, routing::post};

use crate::{AppState, handler};

pub fn router() -> Router<AppState> {
    Router::new().route("/", post(handler::user::create_user))
}
