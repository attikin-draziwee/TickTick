use std::sync::LazyLock;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use regex::Regex;
use serde::Deserialize;
use serde_json::{Value, json};
use validator::{Validate, ValidateEmail};

use crate::{AppState, repository::user::User};

#[derive(Deserialize)]
pub struct UserQueryEmail {
    pub email: String,
}

pub async fn get_by_email(
    State(state): State<AppState>,
    Query(query): Query<UserQueryEmail>,
) -> Result<Json<User>, StatusCode> {
    if let false = query.email.validate_email() {
        return Err(StatusCode::BAD_REQUEST);
    }
    match state.user_service.get_by_email(query.email, state.db).await {
        Ok(u) => Ok(Json(u)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<User>, StatusCode> {
    match state.user_service.get_by_id(id, state.db).await {
        Ok(u) => Ok(Json(u)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

static LOGIN_VALIDATION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^[a-zA-Z0-9_\-]{5,32}$"#).expect("Wrong regular expression"));

#[derive(Debug, Validate, Deserialize)]
pub struct UserCreate {
    #[validate(regex(path = *LOGIN_VALIDATION, message = "Login must be greater then 5 and less 32 symbols. It's support latian letters, digits and symbols '-' and '_'."))]
    login: Option<String>,
    #[validate(email)]
    email: String,
    password: String,
}

pub async fn post(
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
        .create(user.login, user.email, user.password, state.db)
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

#[derive(Debug, Validate, Deserialize)]
pub struct UserUpdate {
    #[validate(regex(path = *LOGIN_VALIDATION, message = "Login must be greater then 5 and less 32 symbols. It's support latian letters, digits and symbols '-' and '_'."))]
    login: Option<String>,
    #[validate(email)]
    email: Option<String>,
    password: Option<String>,
}

pub async fn put(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Json(user_update): Json<UserUpdate>,
) -> impl IntoResponse {
    if user_update.email.is_none() && user_update.login.is_none() && user_update.password.is_none()
    {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status": "error",
                "message": "nothing to do"
            })),
        ));
    };

    if let Err(e) = user_update.validate() {
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
        .update(
            id,
            user_update.login,
            user_update.email,
            user_update.password,
            state.db,
        )
        .await
    {
        Ok(user) => Ok((StatusCode::OK, Json(user))),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status": "error",
                "message": e.to_string(),
            })),
        )),
    }
}
