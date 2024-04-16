use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub admin: bool,
}

use leptos::*;

type UserResource = Resource<(), Result<Option<User>, ServerFnError>>;

impl User {
    pub fn expect() -> UserResource {
        expect_context::<UserResource>()
    }
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
            let user = sqlx::query_as!(User, "SELECT * FROM github_users WHERE id = $1", id)
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
            name: impl Into<String>,
            url: impl Into<String>,
            pool: &PgPool,
        ) -> Result<User> {
            let string_id = github_user_id.to_string();

            let admin = std::env::var("ADMIN_USER_IDS")
                .unwrap()
                .split(',')
                .find(|admin_id| admin_id == &string_id)
                .is_some();

            let user = sqlx::query_as!(
                User,
                "INSERT INTO github_users (id, name, url, admin) VALUES ($1, $2, $3, $4) RETURNING *",
                github_user_id,
                name.into(),
                url.into(),
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
