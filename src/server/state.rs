use axum::extract::FromRef;
use leptos::{get_configuration, LeptosOptions};
use oauth2::basic::BasicClient;
use sqlx::{postgres::PgPoolOptions, PgPool};

#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub leptos_options: LeptosOptions,
    pub oauth_client: BasicClient,
}

impl AppState {
    pub async fn new() -> AppState {
        let conf = get_configuration(None).await.unwrap();
        let leptos_options = conf.leptos_options;

        let pool = PgPoolOptions::new()
            .connect(&std::env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();

        let oauth_client = oauth2::basic::BasicClient::new(
            oauth2::ClientId::new(std::env::var("GITHUB_CLIENT_ID").unwrap()),
            Some(oauth2::ClientSecret::new(
                std::env::var("GITHUB_CLIENT_SECRET").unwrap(),
            )),
            oauth2::AuthUrl::new("https://github.com/login/oauth/authorize".to_string()).unwrap(),
            Some(
                oauth2::TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
                    .unwrap(),
            ),
        )
        .set_redirect_uri(
            oauth2::RedirectUrl::new(std::env::var("GITHUB_REDIRECT_URI").unwrap()).unwrap(),
        );

        AppState {
            leptos_options,
            pool,
            oauth_client,
        }
    }
}
