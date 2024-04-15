#[cfg(feature = "ssr")]
use crate::utils::user::ssr::AuthSession;
use leptos::*;

#[server]
pub async fn create_guestbook_post(content: String) -> Result<(), ServerFnError> {
    let auth_session = expect_context::<AuthSession>();
    let Some(user) = auth_session.current_user else {
        return Err(ServerFnError::new("Unauthorized"));
    };

    let pool = expect_context::<sqlx::PgPool>();

    sqlx::query!(
        "
            INSERT INTO guestbook_posts (
                user_id,
                content,
                published
            ) VALUES (
                $1,
                $2,
                FALSE
            )
        ",
        user.id,
        content
    )
    .execute(&pool)
    .await?;

    Ok(())
}
