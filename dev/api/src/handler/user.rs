use std::sync::LazyLock;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{AppState, repository::user::User};

#[derive(Serialize)]
#[serde(untagged)]
pub enum GetUsers {
    Single(User),
    Multiple(Vec<User>),
}

#[derive(Debug, Validate, Deserialize)]
pub struct UserQueryEmail {
    #[validate(email)]
    pub email: Option<String>,
}

pub async fn get(
    State(state): State<AppState>,
    Query(query): Query<UserQueryEmail>,
) -> Result<Json<GetUsers>, (StatusCode, Json<String>)> {
    let email_validator = query.validate();
    if let Some(email) = query.email {
        if let Err(e) = email_validator {
            return Err((StatusCode::BAD_REQUEST, Json(e.to_string())));
        }

        state
            .user_service
            .get_by_email(email, state.db)
            .await
            .map(|u| Ok(Json(GetUsers::Single(u))))
            .map_err(|e| (StatusCode::NOT_FOUND, Json(e.to_string())))?
    } else {
        state
            .user_service
            .get(false, state.db)
            .await
            .map(|u| Ok(Json(GetUsers::Multiple(u))))
            .map_err(|e| (StatusCode::NOT_FOUND, Json(e.to_string())))?
    }
}

pub async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<User>, StatusCode> {
    state
        .user_service
        .get_by_id(id, state.db)
        .await
        .map(|u| Ok(Json(u)))
        .map_err(|_| StatusCode::NOT_FOUND)?
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
) -> Result<Json<User>, (StatusCode, Json<String>)> {
    if let Err(e) = user.validate() {
        return Err((StatusCode::BAD_REQUEST, Json(e.to_string())));
    }

    state
        .user_service
        .create(user.login, user.email, user.password, state.db)
        .await
        .map(|u| Ok(Json(u)))
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(e.to_string())))?
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
    Path(id): Path<u64>,
    Json(user_update): Json<UserUpdate>,
) -> Result<Json<User>, (StatusCode, Json<String>)> {
    if user_update.email.is_none() && user_update.login.is_none() && user_update.password.is_none()
    {
        return Err((StatusCode::BAD_REQUEST, Json("nothing to do".to_string())));
    };

    if let Err(e) = user_update.validate() {
        return Err((StatusCode::BAD_REQUEST, Json(e.to_string())));
    }

    state
        .user_service
        .update(
            id,
            user_update.login,
            user_update.email,
            user_update.password,
            state.db,
        )
        .await
        .map(|u| Ok(Json(u)))
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(e.to_string())))?
}

pub async fn delete_by_id(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<StatusCode, (StatusCode, Json<String>)> {
    state
        .user_service
        .delete(Some(id), None, state.db)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(e.to_string())))
}
