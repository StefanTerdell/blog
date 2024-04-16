use super::{super::server_fns::LogIn, GithubIconButton};
use crate::github::models::User;

use leptos::*;
use leptos_router::{use_location, ActionForm};

#[component]
pub fn LogInButton(
    #[prop(optional)] small: bool,
    #[prop(optional)] neutral: bool,
) -> impl IntoView {
    let location = use_location();
    let action = create_server_action::<LogIn>();
    let user = User::expect();
    let text = if small {
        "Log in"
    } else {
        "Log in with GitHub"
    };

    view! {
        <ActionForm action=action>
            <input type="hidden" name="redirect_to" value=move || location.pathname/>
            <GithubIconButton text=text loading=user.loading() small=small neutral=neutral/>
        </ActionForm>
    }
}
