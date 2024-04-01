#[cfg(feature = "ssr")] 
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    use axum::Router;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use blog::app::*;
    use blog::fileserv::file_and_error_handler;
    use sqlx::postgres::PgPoolOptions;

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let pool = PgPoolOptions::new().connect(&std::env::var("DATABASE_URL").unwrap()).await.unwrap();

    // let state = AppState {
    //     pool,
    //     leptos_options
    // };

    // build our application with a route
    let app = Router::new()
        .leptos_routes_with_context(&leptos_options, routes, move || {
          provide_context(pool.clone());
        }, App)
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

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
