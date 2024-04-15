#[cfg(feature = "ssr")]
use super::super::utils::expect_admin::expect_admin;
use leptos::*;
#[cfg(feature = "ssr")]
use sqlx::postgres::PgPool;

#[server]
pub async fn delete_blog_post_file(file_name: String) -> Result<(), ServerFnError> {
    let _ = expect_admin();
    let pool = expect_context::<PgPool>();

    sqlx::query!(
        "DELETE FROM blog_post_assets WHERE file_name = $1",
        file_name
    )
    .execute(&pool)
    .await?;

    Ok(())
}
