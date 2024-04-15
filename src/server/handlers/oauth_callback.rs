use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};

use crate::{
    components::github::exchange_code, server::state::AppState, utils::user::ssr::AuthSession,
};
use leptos::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct OAuthCallbackParams {
    pub code: String,
    pub state: String,
}

pub async fn oauth_callback_handler(
    Query(OAuthCallbackParams { code, state }): Query<OAuthCallbackParams>,
    State(AppState {
        pool, oauth_client, ..
    }): State<AppState>,
    auth_session: AuthSession,
) -> impl IntoResponse {
    match exchange_code(state, code, pool, oauth_client, auth_session).await {
        Ok(target) => Redirect::to(&target),
        Err(err) => {
            logging::log!("Failed exchanging code: {err}");
            Redirect::to("/")
        }
    }
}
