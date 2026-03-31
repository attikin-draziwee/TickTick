use axum::{Router, routing::get};

use crate::AppState;
use crate::handler::user;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(user::get_by_email).post(user::post))
        .route("/{:id}", get(user::get_by_id))
}
