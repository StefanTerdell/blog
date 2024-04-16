use super::super::models::BlogPost;
#[cfg(feature = "ssr")]
use crate::github::models::ssr::AuthSession;
use leptos::*;

#[server]
pub async fn get_blog_post(slug: String) -> Result<Option<BlogPost>, ServerFnError> {
    let is_admin = expect_context::<AuthSession>()
        .current_user
        .is_some_and(|u| u.admin);

    let pool = expect_context::<sqlx::PgPool>();
    let post = sqlx::query_as!(
        BlogPost,
        "
            UPDATE blog_posts
            SET views = views + 1
            WHERE slug = $1 AND ($2 OR published)
            RETURNING *
        ",
        slug,
        is_admin
    )
    .fetch_optional(&pool)
    .await?;

    Ok(post)
}
