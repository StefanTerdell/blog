use super::super::server_fns::get_user_from_session;
use leptos::*;

#[component]
pub fn Provider(children: ChildrenFn) -> impl IntoView {
    let user_resource = create_blocking_resource(|| (), move |_| get_user_from_session());

    provide_context(user_resource);

    view! { <Transition>{children()}</Transition> }
}
