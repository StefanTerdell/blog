use leptos::*;

#[cfg(feature = "ssr")]
use super::super::utils::delete_token;

#[cfg(feature = "ssr")]
use crate::utils::user::ssr::AuthSession;

#[cfg(feature = "ssr")]
use sqlx::PgPool;

#[server]
async fn log_out() -> Result<(), ServerFnError> {
    let auth_session = expect_context::<AuthSession>();
    let pool = expect_context::<PgPool>();

    if let Some(user) = &auth_session.current_user {
        delete_token(&user.id, &pool).await?;

        auth_session.logout_user();
    };

    Ok(())
}
