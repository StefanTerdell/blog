use super::{super::server_fns::get_guestbook_posts::get_guestbook_posts, NewPost, Post};
use crate::github::{
    components::{LogInButton, LoggedIn},
    models::UserResource,
};
use leptos::*;

#[component]
pub fn Guestbook() -> impl IntoView {
    let user = expect_context::<UserResource>();
    let posts = create_blocking_resource(move || user(), move |_| get_guestbook_posts());
    let refetch = move || posts.refetch();

    view! {
        <div class="flex flex-col gap-4">
            <LoggedIn fallback=move || view! { <LogInButton/> }>
                <NewPost refetch_posts=move || posts.refetch()/>
            </LoggedIn>
            <Transition>
                {move || match posts() {
                    Some(Ok(posts)) => {
                        posts
                            .into_iter()
                            .map(|post| {
                                view! { <Post post=post refetch_posts=refetch/> }
                            })
                            .collect_view()
                    }
                    Some(Err(err)) => format!("{err:?}").into_view(),
                    None => {
                        view! { <div class="loading loading-spinner mx-auto"></div> }.into_view()
                    }
                }}

            </Transition>
        </div>
    }
}
