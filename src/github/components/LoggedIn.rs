use crate::github::models::User;
use leptos::*;

#[component]
pub fn LoggedIn(#[prop(optional, into)] fallback: ViewFn, children: ChildrenFn) -> impl IntoView {
    let user = User::expect();

    view! {
        <Show when=move || matches!(user(), Some(Ok(Some(_)))) fallback=move || { fallback.run() }>
            {children()}
        </Show>
    }
}
