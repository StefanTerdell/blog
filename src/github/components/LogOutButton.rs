use super::{super::server_fns::LogOut, GithubIconButton};
use crate::github::models::User;
use leptos::*;
use leptos_router::ActionForm;

#[component]
pub fn LogOutButton(
    #[prop(optional)] small: bool,
    #[prop(optional)] neutral: bool,
) -> impl IntoView {
    let action = create_server_action::<LogOut>();
    let user = User::expect();
    let text = if small {
        "Log out"
    } else {
        "Log out from GitHub"
    };

    create_effect(move |_| {
        if let Some(res) = action.value().get() {
            user.set(res.map(|_| None));
        }
    });

    view! {
        <ActionForm action=action>
            <GithubIconButton text=text loading=user.loading() small=small neutral=neutral/>
        </ActionForm>
    }
}
