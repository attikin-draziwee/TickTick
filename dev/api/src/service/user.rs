use std::sync::Arc;

use thiserror::Error;

use crate::repository::user::{RepositoryUser, RepositoryUserError, User};

#[derive(Clone)]
pub struct UserService {
    pub user_repo: Arc<dyn RepositoryUser>,
}

#[derive(Debug, Error)]
pub enum UserError {
    #[error(transparent)]
    RepositoryError(#[from] RepositoryUserError),
    #[error("user not found")]
    UserNotFound,
}

impl UserService {
    pub fn new(user_repo: Arc<dyn RepositoryUser>) -> Self {
        Self { user_repo }
    }

    #[tracing::instrument(skip(self, password), fields(login = ?login, email = %email))]
    pub async fn create(
        &self,
        login: Option<String>,
        email: String,
        password: String,
    ) -> Result<User, UserError> {
        use sha256::digest;

        let crypt_password: String = digest(password);
        self.user_repo
            .create(login, email, crypt_password)
            .await
            .map_err(|e| {
                tracing::error!("{}", &e);
                UserError::RepositoryError(e)
            })
            .inspect(|_| {
                tracing::info!("success created user.");
            })
    }

    #[tracing::instrument(skip(self), fields(email = %email))]
    pub async fn get_by_email(&self, email: String) -> Result<User, UserError> {
        match self.user_repo.get_by_email(email.clone()).await {
            Some(u) => Ok(u),
            None => {
                tracing::error!("user not found.");
                Err(UserError::UserNotFound)
            }
        }
    }

    #[tracing::instrument(skip(self), fields(id = %id))]
    pub async fn get_by_id(&self, id: u64) -> Result<User, UserError> {
        match self.user_repo.get_by_id(id).await {
            Some(u) => Ok(u),
            None => {
                tracing::error!("user not found.");
                Err(UserError::UserNotFound)
            }
        }
    }

    #[tracing::instrument(skip(self, password), fields(id = %id, login = ?login, email = ?email))]
    pub async fn update(
        &self,
        id: u64,
        login: Option<String>,
        email: Option<String>,
        password: Option<String>,
    ) -> Result<User, UserError> {
        tracing::info!("starting update user {}", id);
        let password: Option<String> = if let Some(pass) = password {
            use sha256::digest;
            Some(digest(pass))
        } else {
            None
        };

        match self.user_repo.update(id, login, email, password).await {
            Ok(u) => Ok(u),
            Err(e) => {
                tracing::error!("rollback update {:?}", &e);
                Err(UserError::RepositoryError(e))
            }
        }
    }

    #[tracing::instrument(skip(self), fields(id = ?id, email = ?email))]
    pub async fn delete(&self, id: Option<u64>, email: Option<String>) -> Result<(), UserError> {
        tracing::info!("starting deleting user");

        if id.is_none() && email.is_none() {
            tracing::warn!("request is empty. skip");
            return Err(UserError::UserNotFound);
        }

        if let Some(id) = id {
            tracing::info!("deleting by id.");
            match self.user_repo.delete(id).await {
                Ok(_) => return Ok(()),
                Err(e) => {
                    tracing::error!("can't delete user by id: {:?}", &e);
                    return Err(UserError::RepositoryError(e));
                }
            }
        };

        if let Some(email) = email {
            match self.user_repo.get_by_email(email).await {
                Some(u) => self.user_repo.delete(u.id).await.map_err(|e| {
                    tracing::error!("can't delete user by email: {:?}", &e);
                    UserError::RepositoryError(e)
                }),
                None => {
                    tracing::error!("user not found");
                    Err(UserError::UserNotFound)
                }
            }
        } else {
            tracing::error!("unreachable statement.");
            Err(UserError::UserNotFound)
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn get(&self, strict: bool) -> Result<Vec<User>, UserError> {
        match self.user_repo.get(strict).await {
            Ok(users) => Ok(users),
            Err(e) => {
                tracing::error!("cannot return list of users: {:?}", &e);
                Err(UserError::RepositoryError(e))
            }
        }
    }
}
