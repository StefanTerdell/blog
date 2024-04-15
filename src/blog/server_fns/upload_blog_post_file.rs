#[cfg(feature = "ssr")]
use super::super::utils::expect_admin::expect_admin;
use leptos::*;
use server_fn::codec::{MultipartData, MultipartFormData};

#[server(input = MultipartFormData)]
pub async fn upload_blog_post_file(data: MultipartData) -> Result<usize, ServerFnError> {
    let _ = expect_admin();

    let mut data = data.into_inner().unwrap();
    let mut file_name = vec![];
    let mut file_data = vec![];
    let mut blog_post_id = vec![];

    while let Ok(Some(mut field)) = data.next_field().await {
        let name = field.name().unwrap_or_default().to_string();

        if name == "blog_post_id" {
            while let Ok(Some(chunk)) = field.chunk().await {
                let mut chunk = chunk.into_iter().collect::<Vec<_>>();
                blog_post_id.append(&mut chunk);
            }
        } else if name == "file_name" {
            while let Ok(Some(chunk)) = field.chunk().await {
                let mut chunk = chunk.into_iter().collect::<Vec<_>>();
                file_name.append(&mut chunk);
            }
        } else if name == "file_to_upload" {
            while let Ok(Some(chunk)) = field.chunk().await {
                let mut chunk = chunk.into_iter().collect::<Vec<_>>();
                file_data.append(&mut chunk);
            }
        }
    }

    let blog_post_id = String::from_utf8(blog_post_id)?.parse::<i32>()?;
    let file_name = String::from_utf8(file_name)?;
    let file_size = file_data.len();

    let pool = expect_context::<sqlx::PgPool>();

    sqlx::query!(
        "INSERT INTO blog_post_assets (file_name, data, blog_post_id) VALUES ($1, $2, $3)",
        file_name,
        file_data,
        blog_post_id
    )
    .execute(&pool)
    .await?;

    Ok(file_size)
}
