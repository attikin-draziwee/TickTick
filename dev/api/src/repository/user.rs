use serde::Serialize;
use sqlx::prelude::FromRow;
use thiserror::Error;

pub trait RepositoryUser {
    async fn create(
        &self,
        login: Option<String>,
        email: String,
        password: String,
    ) -> Result<User, RepositoryUserError>;

    async fn get_by_email(&self, email: String) -> Option<User>;

    async fn get_by_id(&self, id: u32) -> Option<User>;

    async fn update(
        &self,
        id: u32,
        login: Option<String>,
        email: Option<String>,
        password: Option<String>,
    ) -> Result<User, RepositoryUserError>;
}

#[derive(Debug, Error)]
pub enum RepositoryUserError {
    #[error(transparent)]
    SQLxError(#[from] sqlx::Error),
    #[error("can't create or update email. Email already exists")]
    EmailAlreadyExists,
    #[error("internal error")]
    InternalError,
}

#[allow(unused)]
#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: u64,
    pub login: Option<String>,
    pub email: String,
    pub password_hash: String,
}
