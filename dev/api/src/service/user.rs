use thiserror::Error;

use crate::repository::user::{RepositoryUser, RepositoryUserError, User};

#[derive(Clone)]
pub struct UserService {}

#[derive(Debug, Error)]
pub enum UserError {
    #[error(transparent)]
    RepositoryError(#[from] RepositoryUserError),
}

impl UserService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn create_user(
        &self,
        login: Option<String>,
        email: String,
        password: String,
        repository: impl RepositoryUser,
    ) -> Result<User, UserError> {
        use sha256::digest;

        let crypt_password: String = digest(password);
        repository
            .create_user(login, email, crypt_password)
            .await
            .map_err(|e| UserError::RepositoryError(e))
    }
}
