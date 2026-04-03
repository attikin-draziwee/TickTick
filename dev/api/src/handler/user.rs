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

#[tracing::instrument(skip(state), fields(query = ?query.email))]
pub async fn get(
    State(state): State<AppState>,
    Query(query): Query<UserQueryEmail>,
) -> Result<Json<GetUsers>, (StatusCode, Json<String>)> {
    let email_validator = query.validate();
    if let Some(email) = query.email {
        tracing::info!("starting search by email.");
        if let Err(e) = email_validator {
            tracing::warn!("email validation error. skip");
            return Err((StatusCode::BAD_REQUEST, Json(e.to_string())));
        }

        state
            .user_service
            .get_by_email(email)
            .await
            .map(|u| {
                tracing::info!("return user: {:?}.", &u);
                Ok(Json(GetUsers::Single(u)))
            })
            .map_err(|e| (StatusCode::NOT_FOUND, Json(e.to_string())))?
    } else {
        state
            .user_service
            .get(false)
            .await
            .map(|u| {
                tracing::info!("return all users.");
                Ok(Json(GetUsers::Multiple(u)))
            })
            .map_err(|e| (StatusCode::NOT_FOUND, Json(e.to_string())))?
    }
}

#[tracing::instrument(skip(state), fields(id = %id))]
pub async fn get_by_id(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<User>, StatusCode> {
    tracing::info!("starting search user by id.");
    state
        .user_service
        .get_by_id(id)
        .await
        .map(|u| {
            tracing::info!("found user {:?}", &u);
            Ok(Json(u))
        })
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

#[tracing::instrument(skip(state, user), fields(login = ?user.login, email = %user.email))]
pub async fn post(
    State(state): State<AppState>,
    Json(user): Json<UserCreate>,
) -> Result<Json<User>, (StatusCode, Json<String>)> {
    tracing::info!("start creating user.");
    if let Err(e) = user.validate() {
        tracing::warn!("it's not valid email, skip.");
        return Err((StatusCode::BAD_REQUEST, Json(e.to_string())));
    }

    state
        .user_service
        .create(user.login, user.email, user.password)
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

#[tracing::instrument(skip(state), fields(id = %id, user_update = ?user_update))]
pub async fn put(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(user_update): Json<UserUpdate>,
) -> Result<Json<User>, (StatusCode, Json<String>)> {
    if user_update.email.is_none() && user_update.login.is_none() && user_update.password.is_none()
    {
        tracing::warn!("empty request. skip");
        return Err((StatusCode::BAD_REQUEST, Json("nothing to do".to_string())));
    };

    if let Err(e) = user_update.validate() {
        tracing::warn!("email not valid. skip");
        return Err((StatusCode::BAD_REQUEST, Json(e.to_string())));
    }

    state
        .user_service
        .update(
            id,
            user_update.login,
            user_update.email,
            user_update.password,
        )
        .await
        .map(|u| {
            tracing::info!("updated user");
            Ok(Json(u))
        })
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(e.to_string())))?
}

#[tracing::instrument(skip(state), fields(id = %id))]
pub async fn delete_by_id(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<StatusCode, (StatusCode, Json<String>)> {
    state
        .user_service
        .delete(Some(id), None)
        .await
        .map(|_| {
            tracing::info!("user {} successfully deleted.", id);
            StatusCode::OK
        })
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(e.to_string())))
}
