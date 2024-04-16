use super::super::models;
#[cfg(feature = "ssr")]
use crate::github::models::ssr::AuthSession;
use leptos::*;

#[server]
pub async fn get_guestbook_posts() -> Result<Vec<models::GuestbookPost>, ServerFnError> {
    let user = expect_context::<AuthSession>().current_user;
    let pool = expect_context::<sqlx::PgPool>();
    let posts = sqlx::query_as!(
        models::GuestbookPost,
        "
            SELECT 
                guestbook_posts.*,
                github_users.name AS user_name,
                github_users.url AS user_url
            FROM guestbook_posts 
            JOIN github_users ON guestbook_posts.user_id = github_users.id
            WHERE $1 OR guestbook_posts.published=true OR guestbook_posts.user_id = $2
            ORDER BY guestbook_posts.created_time DESC
        ",
        user.as_ref().is_some_and(|u| u.admin),
        user.as_ref().map(|u| u.id).unwrap_or(-1)
    )
    .fetch_all(&pool)
    .await?;

    Ok(posts)
}
