#[cfg(feature = "ssr")]
use crate::github::models::ssr::AuthSession;
use leptos::*;

#[server]
pub async fn delete_post(post_id: i32) -> Result<(), ServerFnError> {
    let auth_session = expect_context::<AuthSession>();
    let Some(user) = auth_session.current_user else {
        return Err(ServerFnError::new("Unauthorized"));
    };

    let pool = expect_context::<sqlx::PgPool>();

    let _n = if user.admin {
        sqlx::query!("DELETE FROM guestbook_posts WHERE id = $1", post_id)
            .execute(&pool)
            .await?
    } else {
        sqlx::query!(
            "DELETE FROM guestbook_posts WHERE id = $1 AND user_id = $2",
            post_id,
            user.id
        )
        .execute(&pool)
        .await?
    };

    Ok(())
}
