use super::super::models::BlogPost;
#[cfg(feature = "ssr")]
use super::super::utils::expect_admin::expect_admin;
use leptos::*;

#[server]
pub async fn get_blog_post_or_create_new(slug: String) -> Result<BlogPost, ServerFnError> {
    let _ = expect_admin();

    let pool = expect_context::<sqlx::PgPool>();
    let post = sqlx::query_as!(BlogPost, "SELECT * FROM blog_posts WHERE slug = $1", &slug)
        .fetch_optional(&pool)
        .await?;

    if let Some(post) = post {
        return Ok(post);
    }

    let post = sqlx::query_as!(
        BlogPost,
        "
            INSERT INTO blog_posts (slug, title, md_content, html_content, published, views)
            VALUES ($1, '', '', '', FALSE, 0)
            RETURNING *
        ",
        &slug
    )
    .fetch_one(&pool)
    .await?;

    Ok(post)
}
