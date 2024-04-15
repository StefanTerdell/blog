#[cfg(feature = "ssr")]
use crate::utils::user::ssr::AuthSession;
use leptos::*;

#[server]
pub async fn publish_post(post_id: i32) -> Result<(), ServerFnError> {
    let auth_session = expect_context::<AuthSession>();
    if !auth_session.current_user.is_some_and(|u| u.admin) {
        return Err(ServerFnError::new("Unauthorized"));
    };

    let pool = expect_context::<sqlx::PgPool>();

    sqlx::query!(
        "UPDATE guestbook_posts SET published=true WHERE id = $1",
        post_id
    )
    .execute(&pool)
    .await?;

    Ok(())
}
