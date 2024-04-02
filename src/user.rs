use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub admin: bool,
}

#[cfg(feature = "ssr")]
pub mod ssr {
    use super::User;

    use anyhow::{bail, Result};
    use axum::async_trait;
    use axum_session_auth::{Authentication, SessionPgPool};
    use sqlx::PgPool;

    pub type AuthSession = axum_session_auth::AuthSession<User, i32, SessionPgPool, PgPool>;

    impl User {
        pub async fn get_by_id(id: i32, pool: &PgPool) -> Result<Option<User>> {
            let user = sqlx::query_as!(
                User,
                "SELECT id, email, admin FROM \"user\" WHERE id = $1",
                id
            )
            .fetch_one(pool)
            .await;

            match user {
                Ok(user) => Ok(Some(user)),
                Err(err) => match err {
                    sqlx::Error::RowNotFound => Ok(None),
                    err => Err(err.into()),
                },
            }
        }

        pub async fn get_by_email(email: impl Into<String>, pool: &PgPool) -> Result<Option<User>> {
            let user = sqlx::query_as!(
                User,
                "SELECT id, email, admin FROM \"user\" WHERE email = $1",
                email.into()
            )
            .fetch_one(pool)
            .await;

            match user {
                Ok(user) => Ok(Some(user)),
                Err(err) => match err {
                    sqlx::Error::RowNotFound => Ok(None),
                    err => Err(err.into()),
                },
            }
        }

        pub async fn register(
            github_user_id: &i32,
            email: impl Into<String>,
            pool: &PgPool,
        ) -> Result<User> {
            let github_user_id = github_user_id.to_string();

            let admin = std::env::var("ADMIN_USER_IDS")
                .unwrap()
                .split(',')
                .find(|admin_id| admin_id == &github_user_id)
                .is_some();

            let user = sqlx::query_as!(
                User,
                "INSERT INTO \"user\" (email, admin, created_at) VALUES ($1, $2, EXTRACT(epoch FROM NOW())::BIGINT) RETURNING id, email, admin",
                email.into(),
                admin
            ).fetch_one(pool).await?;

            Ok(user)
        }
    }

    #[async_trait]
    impl Authentication<User, i32, PgPool> for User {
        async fn load_user(userid: i32, pool: Option<&PgPool>) -> Result<User> {
            let pool = pool.unwrap();

            let user = User::get_by_id(userid, &pool).await?;

            if let Some(user) = user {
                Ok(user)
            } else {
                bail!("No user found")
            }
        }

        fn is_authenticated(&self) -> bool {
            true
        }

        fn is_active(&self) -> bool {
            true
        }

        fn is_anonymous(&self) -> bool {
            false
        }
    }
}
