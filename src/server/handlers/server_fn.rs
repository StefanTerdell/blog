use crate::{github::models::ssr::AuthSession, server::state::AppState};
use axum::{
    body::Body,
    extract::{Request, State},
    response::IntoResponse,
};
use leptos::*;
use leptos_axum::handle_server_fns_with_context;

pub async fn server_fn_handler(
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
