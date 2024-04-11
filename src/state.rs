use axum::extract::FromRef;
use leptos::LeptosOptions;
use oauth2::basic::BasicClient;
use sqlx::PgPool;

#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub leptos_options: LeptosOptions,
    pub oauth_client: BasicClient,
}
