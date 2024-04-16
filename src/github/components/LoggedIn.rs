use super::super::models::UserResource;
use leptos::*;

#[component]
pub fn LoggedIn(#[prop(optional, into)] fallback: ViewFn, children: ChildrenFn) -> impl IntoView {
    let user = expect_context::<UserResource>();

    view! {
        <Show when=move || matches!(user(), Some(Ok(Some(_)))) fallback=move || { fallback.run() }>
            {children()}
        </Show>
    }
}
