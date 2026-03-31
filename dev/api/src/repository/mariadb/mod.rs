use sqlx::MySqlPool;

use crate::repository::user::{RepositoryUser, RepositoryUserError, User};

impl RepositoryUser for MySqlPool {
    async fn create_user(
        &self,
        login: Option<String>,
        email: String,
        password: String,
    ) -> Result<User, RepositoryUserError> {
        let mut tx = self.begin().await?;

        sqlx::query!("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ")
            .execute(self)
            .await?;

        let insert_user = sqlx::query!(
            "INSERT `user` (login, email, password_hash) VALUES (?, ?, ?);",
            login,
            email.to_string(),
            password.to_string()
        )
        .execute(&mut *tx)
        .await
        .map_err(|sql_err| match sql_err {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                RepositoryUserError::EmailAlreadyExists
            }
            _ => RepositoryUserError::SQLxError(sql_err),
        });

        if let Err(e) = insert_user {
            tx.rollback().await.ok();
            return Err(e);
        }

        let user = sqlx::query_as!(User, "SELECT * FROM `user` WHERE email = ?", email)
            .fetch_one(&mut *tx)
            .await;

        return match user {
            Ok(user) => {
                tx.commit().await?;
                Ok(user)
            }
            Err(_) => {
                tx.rollback().await.ok();
                Err(RepositoryUserError::InternalError)
            }
        };
    }
}
