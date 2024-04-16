use crate::utils::user::User;
use leptos::*;

#[cfg(feature = "ssr")]
use crate::utils::user::ssr::AuthSession;

#[server]
pub async fn get_user_from_session() -> Result<Option<User>, ServerFnError> {
    let auth_session = expect_context::<AuthSession>();

    Ok(auth_session.current_user)
}
