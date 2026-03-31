use sqlx::{Connection, MySqlPool};

use crate::repository::user::{RepositoryUser, RepositoryUserError, User, UserRow};

impl RepositoryUser for MySqlPool {
    async fn create(
        &self,
        login: Option<String>,
        email: String,
        password: String,
    ) -> Result<User, RepositoryUserError> {
        let mut conn = self.acquire().await?;

        sqlx::query!("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ")
            .execute(&mut *conn)
            .await?;

        let mut tx = conn.begin().await?;

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

    async fn get_by_email(&self, email: String) -> Option<User> {
        sqlx::query_as!(UserRow, "SELECT * FROM `user` WHERE email = ?", email)
            .fetch_one(self)
            .await
            .map(|user_row| User::from(user_row))
            .ok()
    }

    async fn get_by_id(&self, id: u32) -> Option<User> {
        sqlx::query_as!(UserRow, "SELECT * FROM `user` WHERE id = ?", id)
            .fetch_one(self)
            .await
            .map(User::from)
            .ok()
    }

    async fn update(
        &self,
        id: u32,
        login: Option<String>,
        email: Option<String>,
        password: Option<String>,
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
            _ => RepositoryUserError::SQLxError(sql_err),
        });

        if let Err(e) = updated_user {
            tx.rollback().await.ok();
            return Err(e);
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
}
