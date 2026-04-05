use async_trait::async_trait;
use sqlx::{Connection, MySqlPool};

use crate::repository::user::{RepositoryUser, RepositoryUserError, User, UserRow};

#[async_trait]
impl RepositoryUser for MySqlPool {
    async fn create(
        &self,
        login: Option<&str>,
        email: &str,
        password: &str,
    ) -> Result<User, RepositoryUserError> {
        let mut conn = self.acquire().await?;

        sqlx::query!("SET TRANSACTION ISOLATION LEVEL READ COMMITTED")
            .execute(&mut *conn)
            .await?;

        let mut tx = conn.begin().await?;

        let insert_user = sqlx::query!(
            "INSERT `user` (login, email, password_hash) VALUES (?, ?, ?);",
            login,
            email,
            password
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

        let user = sqlx::query_as!(UserRow, "SELECT * FROM `user` WHERE email = ?", email)
            .fetch_one(&mut *tx)
            .await;

        return match user {
            Ok(user) => {
                tx.commit().await?;
                Ok(User::from(user))
            }
            Err(_) => {
                tx.rollback().await.ok();
                Err(RepositoryUserError::InternalError)
            }
        };
    }

    async fn get(&self, strict: bool) -> Result<Vec<User>, RepositoryUserError> {
        let rows_result = match strict {
            false => {
                sqlx::query_as!(UserRow, "SELECT * FROM `user`;")
                    .fetch_all(self)
                    .await
            }
            true => {
                sqlx::query_as!(UserRow, "SELECT * FROM `user` WHERE is_deleted = FALSE;")
                    .fetch_all(self)
                    .await
            }
        };

        match rows_result {
            Err(e) => Err(RepositoryUserError::SQLxError(e)),
            Ok(rows) => Ok(rows.into_iter().map(User::from).collect()),
        }
    }

    async fn get_by_email(&self, email: &str) -> Option<User> {
        sqlx::query_as!(
            UserRow,
            "SELECT * FROM `user` WHERE email = ? AND is_deleted = FALSE",
            email
        )
        .fetch_one(self)
        .await
        .map(User::from)
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryUserError::UserNotFound,
            _ => RepositoryUserError::SQLxError(e),
        })
        .ok()
    }

    async fn get_by_id(&self, id: u64) -> Option<User> {
        sqlx::query_as!(
            UserRow,
            "SELECT * FROM `user` WHERE id = ? AND is_deleted = FALSE",
            id
        )
        .fetch_one(self)
        .await
        .map(User::from)
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryUserError::UserNotFound,
            _ => RepositoryUserError::SQLxError(e),
        })
        .ok()
    }

    async fn update(
        &self,
        id: u64,
        login: Option<&str>,
        email: Option<&str>,
        password: Option<&str>,
    ) -> Result<User, RepositoryUserError> {
        let mut conn = self.acquire().await?;

        sqlx::query!("SET TRANSACTION ISOLATION LEVEL SERIALIZABLE")
            .execute(&mut *conn)
            .await?;

        let mut tx = conn.begin().await?;

        let updated_user = sqlx::query_as!(
            User,
            r#"UPDATE `user`
            SET
                login = COALESCE(?, login),
                email = COALESCE(?, email),
                password_hash = COALESCE(?, password_hash)
            WHERE
                id = ?
                AND
                is_deleted = FALSE
           ;"#,
            login,
            email,
            password,
            id
        )
        .execute(&mut *tx)
        .await
        .map_err(|sql_err| match sql_err {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                RepositoryUserError::EmailAlreadyExists
            }
            sqlx::Error::RowNotFound => RepositoryUserError::UserNotFound,
            _ => RepositoryUserError::SQLxError(sql_err),
        });

        match updated_user {
            Ok(rows) => {
                if rows.rows_affected() == 0 {
                    tx.rollback().await.ok();
                    return Err(RepositoryUserError::UserNotFound);
                }
            }
            Err(e) => {
                tx.rollback().await.ok();
                return Err(e);
            }
        }

        let user = sqlx::query_as!(UserRow, "SELECT * FROM `user` WHERE id = ?", id)
            .fetch_one(&mut *tx)
            .await;

        match user {
            Ok(u) => {
                tx.commit().await.ok();
                Ok(User::from(u))
            }
            Err(e) => {
                tx.rollback().await.ok();
                Err(RepositoryUserError::SQLxError(e))
            }
        }
    }

    async fn delete(&self, id: u64) -> Result<(), RepositoryUserError> {
        let result = sqlx::query!(
            "UPDATE `user` SET is_deleted = 1 WHERE id = ? AND is_deleted = FALSE",
            id
        )
        .execute(self)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryUserError::UserNotFound,
            _ => RepositoryUserError::SQLxError(e),
        })?;

        if result.rows_affected() == 0 {
            Err(RepositoryUserError::UserNotFound)
        } else {
            Ok(())
        }
    }
}
