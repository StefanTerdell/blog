#[cfg(feature = "ssr")]
use leptos::ServerFnError;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Payload {
    access_token: String,
}

#[cfg(feature = "ssr")]
pub async fn delete_token(user_id: &i32, pool: &sqlx::PgPool) -> Result<(), ServerFnError> {
    let result = sqlx::query_as!(
        Payload,
        "DELETE FROM github_tokens WHERE user_id = $1 RETURNING access_token",
        user_id
    )
    .fetch_optional(pool)
    .await?;

    if let Some(payload) = result {
        reqwest::Client::new()
            .delete(format!(
                "https://api.github.com/applications/{}/token",
                std::env::var("GITHUB_CLIENT_SECRET").unwrap()
            ))
            .header("Accept", "application/vnd.github+json")
            .basic_auth(
                std::env::var("GITHUB_CLIENT_ID").unwrap().to_string(),
                Some(std::env::var("GITHUB_CLIENT_SECRET").unwrap().to_string()),
            )
            .header("X-GitHub-Api-Version", "2022-11-28")
            .json(&payload)
            .send()
            .await?;
    }

    Ok(())
}
