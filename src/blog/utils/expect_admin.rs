use crate::github::models::ssr::AuthSession;
use crate::github::models::User;
use leptos::*;

pub fn expect_admin() -> Result<User, ServerFnError> {
    let auth_session = expect_context::<AuthSession>();

    let user = auth_session.current_user;

    if let Some(user) = user {
        if user.admin {
            return Ok(user);
        }
    };

    Err(ServerFnError::new("Unauthorized"))
}
