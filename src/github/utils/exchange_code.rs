#[cfg(feature = "ssr")]
use super::delete_token;

#[cfg(feature = "ssr")]
use anyhow::bail;

#[cfg(feature = "ssr")]
use oauth2::{reqwest::async_http_client, AuthorizationCode, TokenResponse};

#[cfg(feature = "ssr")]
use serde::Deserialize;

#[cfg(feature = "ssr")]
#[derive(Deserialize, Debug)]
struct GithubUser {
    id: i32,
    html_url: String,
    name: Option<String>,
    login: String,
}

#[cfg(feature = "ssr")]
use crate::github::models::User;

#[cfg(feature = "ssr")]
pub async fn exchange_code(
    provided_csrf: String,
    code: String,
    pool: sqlx::PgPool,
    oauth_client: oauth2::basic::BasicClient,
    auth_session: crate::github::models::ssr::AuthSession,
) -> anyhow::Result<String> {
    #[derive(Deserialize, Debug)]
    struct CsrfRow {
        redirect_to: String,
    }

    let csrf_row = sqlx::query_as!(
        CsrfRow,
        "DELETE FROM csrf_tokens WHERE csrf_token = $1 RETURNING redirect_to",
        &provided_csrf
    )
    .fetch_optional(&pool)
    .await?;

    let Some(CsrfRow { redirect_to }) = csrf_row else {
        bail!("No matching CSRF token found");
    };

    let token_response = oauth_client
        .exchange_code(AuthorizationCode::new(code.clone()))
        .request_async(async_http_client)
        .await?;

    let access_token = token_response.access_token().secret();

    let github_user = reqwest::Client::new()
        .get("https://api.github.com/user")
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "guestbook")
        .bearer_auth(&access_token)
        .send()
        .await?
        .json::<GithubUser>()
        .await?;

    let user = match User::get_by_id(github_user.id, &pool).await? {
        Some(user) => user,
        None => {
            User::register(
                &github_user.id,
                &github_user.name.unwrap_or(github_user.login),
                &github_user.html_url,
                &pool,
            )
            .await?
        }
    };

    delete_token(&user.id, &pool)
        .await
        .map_err(|err| anyhow::anyhow!("{err}"))?;

    auth_session.login_user(user.id);

    drop(auth_session);

    sqlx::query!(
        "INSERT INTO github_tokens (user_id, access_token) VALUES ($1, $2)",
        &user.id,
        &access_token,
    )
    .execute(&pool)
    .await?;

    Ok(redirect_to)
}
