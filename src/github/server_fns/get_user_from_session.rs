use crate::github::models::User;
use leptos::*;

#[server]
pub async fn get_user_from_session() -> Result<Option<User>, ServerFnError> {
    use crate::github::models::ssr::AuthSession;

    let auth_session = use_context::<AuthSession>();

    Ok(auth_session.and_then(|s| s.current_user))
}
