use axum::{
    Router,
    extract::Request,
    middleware::{self, Next},
    response::Response,
    routing::get,
};
use tracing::Instrument;

use crate::{AppState, handler, router};
mod user;

pub fn main_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(handler::health::health))
        .nest("/user", router::user::router())
        .layer(middleware::from_fn(request_id_middleware))
        .with_state(state)
}

pub async fn request_id_middleware(request: Request, next: Next) -> Response {
    let request_id = uuid::Uuid::new_v4().to_string();

    let span = tracing::info_span!("req: ", request_id);

    tracing::info!("incoming {} {}", request.method(), request.uri());

    next.run(request).instrument(span).await
}
