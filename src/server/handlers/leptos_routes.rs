use crate::{app::*, server::state::AppState, utils::user::ssr::AuthSession};
use axum::{
    body::Body,
    extract::{Request, State},
    response::IntoResponse,
};
use leptos::*;
use leptos_axum::render_app_async_with_context;

pub async fn leptos_routes_handler(
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
