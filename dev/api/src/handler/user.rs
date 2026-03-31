use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;
use serde_json::{Value, json};
use validator::Validate;

use crate::{AppState, repository::user::User};

#[derive(Debug, Validate, Deserialize)]
pub struct UserCreate {
    login: Option<String>,
    #[validate(email)]
    email: String,
    password: String,
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(user): Json<UserCreate>,
) -> Result<(StatusCode, Json<User>), (StatusCode, Json<Value>)> {
    if let Err(e) = user.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status": "error",
                "message": e.to_string(),
            })),
        ));
    }

    match state
        .user_service
        .create_user(user.login, user.email, user.password, state.db)
        .await
    {
        Ok(user) => Ok((StatusCode::CREATED, Json(user))),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status": "error",
                "message": e.to_string(),
            })),
        )),
    }
}
