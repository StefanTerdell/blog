use leptos::*;

#[cfg(feature = "ssr")]
use oauth2::{CsrfToken, Scope};

#[server]
pub async fn log_in(redirect_to: String) -> Result<(), ServerFnError> {
    let oauth_client = expect_context::<oauth2::basic::BasicClient>();
    let pool = expect_context::<sqlx::PgPool>();

    let (url, csrf_token) = oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("read:user".to_string()))
        .url();

    sqlx::query!(
        "INSERT INTO csrf_tokens (csrf_token, redirect_to) VALUES ($1, $2)",
        csrf_token.secret(),
        redirect_to
    )
    .execute(&pool)
    .await?;

    let url = url.to_string();

    leptos_axum::redirect(&url);

    Ok(())
}
