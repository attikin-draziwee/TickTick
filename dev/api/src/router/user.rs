use axum::{Router, routing::get};

use crate::AppState;
use crate::handler::user;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(user::get).post(user::post))
        .route(
            "/{:id}",
            get(user::get_by_id)
                .put(user::put)
                .delete(user::delete_by_id),
        )
}
