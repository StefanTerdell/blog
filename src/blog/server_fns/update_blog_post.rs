use super::super::models::{UpdateBlogPostPayload, UpdateBlogPostResult};
#[cfg(feature = "ssr")]
use super::super::utils::expect_admin::expect_admin;
#[cfg(feature = "ssr")]
use super::super::utils::markdown_to_html::markdown_to_html;
#[cfg(feature = "ssr")]
use chrono::{DateTime, Utc};
use leptos::*;
#[cfg(feature = "ssr")]
use serde::Deserialize;

#[server]
async fn update_blog_post(
    post: UpdateBlogPostPayload,
) -> Result<UpdateBlogPostResult, ServerFnError> {
    let _ = expect_admin();
    let published = post.published.is_some_and(|p| p == "on");
    let pool = expect_context::<sqlx::PgPool>();
    let html_content = markdown_to_html(&post.md_content);

    #[derive(Deserialize)]
    struct Status {
        published: bool,
        published_time: DateTime<Utc>,
    }

    let status = sqlx::query_as!(
        Status,
        "
            SELECT published, published_time
            FROM blog_posts
            WHERE id = $1
        ",
        &post.id
    )
    .fetch_one(&pool)
    .await?;

    let now = chrono::offset::Utc::now();

    let (published_time, edited_time) = if !status.published && published {
        (now.clone(), None)
    } else {
        (status.published_time, Some(now.clone()))
    };

    #[derive(Deserialize)]
    struct Returning {
        slug: String,
    }

    let result = sqlx::query_as!(
        Returning,
        "
            UPDATE blog_posts 
            SET 
                slug = $2,
                title = $3,
                md_content = $4,
                html_content = $5,
                published = $6,
                published_time = $7,
                edited_time = $8
            WHERE id = $1
            RETURNING slug
        ",
        &post.id,
        &post.slug,
        &post.title,
        &post.md_content,
        html_content,
        published,
        published_time,
        edited_time
    )
    .fetch_one(&pool)
    .await?;

    Ok(UpdateBlogPostResult {
        saved_time: now,
        slug: result.slug,
    })
}
