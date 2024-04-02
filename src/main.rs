#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    use axum::body::Body;
    use axum::extract::{Request, State};
    use axum::response::IntoResponse;
    use axum::routing::get;
    use axum::Router;
    use axum_session::{Key, SessionConfig, SessionLayer, SessionPgPool, SessionStore};
    use axum_session_auth::{AuthConfig, AuthSessionLayer};
    use blog::app::*;
    use blog::fileserv::file_and_error_handler;
    use blog::state::AppState;
    use blog::user::{ssr::AuthSession, User};
    use leptos::*;
    use leptos_axum::{
        generate_route_list, handle_server_fns_with_context, render_app_async_with_context,
        LeptosRoutes,
    };
    use sqlx::postgres::PgPoolOptions;
    use sqlx::PgPool;

    async fn server_fn_handler(
        State(app_state): State<AppState>,
        auth_session: AuthSession,
        request: Request<Body>,
    ) -> impl IntoResponse {
        handle_server_fns_with_context(
            move || {
                provide_context(auth_session.clone());
                provide_context(app_state.oauth_client.clone());
                provide_context(app_state.pool.clone());
            },
            request,
        )
        .await
    }

    async fn leptos_routes_handler(
        State(app_state): State<AppState>,
        auth_session: AuthSession,
        request: Request<Body>,
    ) -> axum::response::Response {
        let handler = render_app_async_with_context(
            app_state.leptos_options.clone(),
            move || {
                provide_context(auth_session.clone());
                provide_context(app_state.oauth_client.clone());
                provide_context(app_state.pool.clone());
            },
            move || view! { <App/> },
        );

        handler(request).await.into_response()
    }

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let pool = PgPoolOptions::new()
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let auth_config = AuthConfig::<i32>::default();

    let session_config = SessionConfig::default()
        .with_table_name("sessions")
        .with_key(Key::generate())
        .with_database_key(Key::generate());

    let session_store =
        SessionStore::<SessionPgPool>::new(Some(pool.clone().into()), session_config)
            .await
            .unwrap();

    sqlx::migrate!().run(&pool).await.unwrap();

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

    let app_state = AppState {
        leptos_options,
        pool: pool.clone(),
        oauth_client,
    };

    let app = Router::new()
        .route(
            "/api/*fn_name",
            get(server_fn_handler).post(server_fn_handler),
        )
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .layer(
            AuthSessionLayer::<User, i32, SessionPgPool, PgPool>::new(Some(pool.clone()))
                .with_config(auth_config),
        )
        .layer(SessionLayer::new(session_store))
        .fallback(file_and_error_handler)
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
