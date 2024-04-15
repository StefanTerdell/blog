#[cfg(feature = "ssr")]
use super::super::utils::expect_admin::expect_admin;
use leptos::*;
#[cfg(feature = "ssr")]
use serde::Deserialize;
#[cfg(feature = "ssr")]
use sqlx::postgres::PgPool;

#[server]
pub async fn get_blog_post_file_list(blog_post_id: i32) -> Result<Vec<String>, ServerFnError> {
    let _ = expect_admin()?;
    let pool = expect_context::<PgPool>();

    #[derive(Deserialize)]
    struct Row {
        file_name: String,
    }

    let rows = sqlx::query_as!(
        Row,
        "SELECT file_name FROM blog_post_assets WHERE blog_post_id = $1",
        blog_post_id
    )
    .fetch_all(&pool)
    .await?;

    Ok(rows.into_iter().map(|r| r.file_name).collect())
}
