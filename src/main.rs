#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    use axum::{routing::get, Router};
    use axum_session::{Key, SessionConfig, SessionLayer, SessionPgPool, SessionStore};
    use axum_session_auth::{AuthConfig, AuthSessionLayer};
    use blog::{
        app::*,
        github::models::User,
        server::{
            handlers::{
                blog_file::blog_file_handler, file_and_error::file_and_error_handler,
                leptos_routes::leptos_routes_handler, oauth_callback::oauth_callback_handler,
                server_fn::server_fn_handler,
            },
            state::AppState,
        },
    };
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use sqlx::PgPool;

    let app_state = AppState::new().await;
    let addr = app_state.leptos_options.site_addr.clone();
    let auth_config = AuthConfig::<i32>::default();

    let session_config = SessionConfig::default()
        .with_table_name("sessions")
        .with_key(Key::generate())
        .with_database_key(Key::generate());

    let session_store =
        SessionStore::<SessionPgPool>::new(Some(app_state.pool.clone().into()), session_config)
            .await
            .unwrap();

    sqlx::migrate!().run(&app_state.pool).await.unwrap();

    let app = Router::new()
        .route(
            "/api/*fn_name",
            get(server_fn_handler).post(server_fn_handler),
        )
        .leptos_routes_with_handler(generate_route_list(App), get(leptos_routes_handler))
        .route("/blog-files/:file_name", get(blog_file_handler))
        .route("/callback", get(oauth_callback_handler))
        .layer(
            AuthSessionLayer::<User, i32, SessionPgPool, PgPool>::new(Some(app_state.pool.clone()))
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
