use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use thiserror::Error;

pub trait RepositoryUser {
    async fn create(
        &self,
        login: Option<String>,
        email: String,
        password: String,
    ) -> Result<User, RepositoryUserError>;

    async fn get(&self, strict: bool) -> Result<Vec<User>, RepositoryUserError>;

    async fn get_by_email(&self, email: String) -> Option<User>;

    async fn get_by_id(&self, id: u64) -> Option<User>;

    async fn update(
        &self,
        id: u64,
        login: Option<String>,
        email: Option<String>,
        password: Option<String>,
    ) -> Result<User, RepositoryUserError>;

    async fn delete(&self, id: u64) -> Result<(), RepositoryUserError>;
}

#[derive(Debug, Error)]
pub enum RepositoryUserError {
    #[error(transparent)]
    SQLxError(#[from] sqlx::Error),
    #[error("can't create or update email. Email already exists")]
    EmailAlreadyExists,
    #[error("internal error")]
    InternalError,
    #[error("user not found")]
    UserNotFound,
}

#[allow(unused)]
#[derive(Debug, Serialize, FromRow)]
pub struct UserRow {
    pub id: u64,
    pub login: Option<String>,
    pub email: String,
    pub password_hash: String,
    pub is_deleted: i8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub login: Option<String>,
    pub email: String,
    pub password_hash: String,
    pub is_deleted: bool,
}

impl From<UserRow> for User {
    fn from(value: UserRow) -> Self {
        Self {
            id: value.id,
            login: value.login,
            email: value.email,
            password_hash: value.password_hash,
            is_deleted: match value.is_deleted {
                0 => false,
                _ => true,
            },
        }
    }
}
