use thiserror::Error;

use crate::repository::user::{RepositoryUser, RepositoryUserError, User};

#[derive(Clone)]
pub struct UserService {}

#[derive(Debug, Error)]
pub enum UserError {
    #[error(transparent)]
    RepositoryError(#[from] RepositoryUserError),
    #[error("user not found")]
    UserNotFound,
}

impl UserService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn create(
        &self,
        login: Option<String>,
        email: String,
        password: String,
        repository: impl RepositoryUser,
    ) -> Result<User, UserError> {
        use sha256::digest;

        let crypt_password: String = digest(password);
        repository
            .create(login, email, crypt_password)
            .await
            .map_err(|e| UserError::RepositoryError(e))
    }

    pub async fn get_by_email(
        &self,
        email: String,
        repository: impl RepositoryUser,
    ) -> Result<User, UserError> {
        match repository.get_by_email(email.clone()).await {
            Some(u) => Ok(u),
            None => Err(UserError::UserNotFound),
        }
    }

    pub async fn get_by_id(
        &self,
        id: u32,
        repository: impl RepositoryUser,
    ) -> Result<User, UserError> {
        match repository.get_by_id(id).await {
            Some(u) => Ok(u),
            None => Err(UserError::UserNotFound),
        }
    }
}
