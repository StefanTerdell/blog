use super::super::models::BlogPostListItem;
#[cfg(feature = "ssr")]
use crate::github::models::ssr::AuthSession;
use leptos::*;

#[server]
pub async fn get_blog_post_list() -> Result<Vec<BlogPostListItem>, ServerFnError> {
    let is_admin = expect_context::<AuthSession>()
        .current_user
        .is_some_and(|u| u.admin);

    let pool = expect_context::<sqlx::PgPool>();
    let posts = sqlx::query_as!(
        BlogPostListItem,
        "
            SELECT 
                slug,
                title,
                views,
                published_time
            FROM blog_posts
            WHERE $1 OR published
            ORDER BY published_time DESC
        ",
        is_admin
    )
    .fetch_all(&pool)
    .await?;

    Ok(posts)
}
