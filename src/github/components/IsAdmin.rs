use super::super::models::UserResource;
use crate::utils::user::User;
use leptos::*;

#[component]
pub fn IsAdmin(#[prop(optional, into)] fallback: ViewFn, children: ChildrenFn) -> impl IntoView {
    let user = expect_context::<UserResource>();

    view! {
        <Show
            when=move || matches!(user(), Some(Ok(Some(User { admin: true, .. }))))
            fallback=move || { fallback.run() }
        >
            {children()}
        </Show>
    }
}
