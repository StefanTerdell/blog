use crate::server::state::AppState;
use axum::{
    body::Body,
    extract::{Path, State},
    response::IntoResponse,
};
use http::{header, StatusCode};
use serde::Deserialize;

pub async fn blog_file_handler(
    Path(file_name): Path<String>,
    State(AppState { pool, .. }): State<AppState>,
) -> impl IntoResponse {
    #[derive(Deserialize)]
    struct DbResponse {
        data: Vec<u8>,
    }

    let db_response = sqlx::query_as!(
        DbResponse,
        "SELECT data FROM blog_post_assets WHERE file_name = $1",
        file_name
    )
    .fetch_optional(&pool)
    .await;

    match db_response {
        Ok(Some(DbResponse { data })) => (
            [(
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{file_name}\""),
            )],
            Body::from(data),
        )
            .into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    }
}
