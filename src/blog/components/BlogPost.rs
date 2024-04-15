use super::{super::server_fns::get_blog_post::get_blog_post, RenderBlogPost::RenderBlogPost};
use leptos::*;
use leptos_router::use_params_map;

#[component]
pub fn BlogPost() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.with(|params| params.get("slug").cloned().unwrap_or_default());
    let post = create_blocking_resource(move || slug(), move |slug| get_blog_post(slug));

    view! {
        <Transition>
            {move || {
                match post.get() {
                    Some(Ok(Some(post))) => view! { <RenderBlogPost post=post/> }.into_view(),
                    Some(Ok(None)) => "No post found".into_view(),
                    Some(Err(err)) => format!("{err:?}").into_view(),
                    None => {
                        view! { <div class="loading loading-spinner mx-auto"></div> }.into_view()
                    }
                }
            }}

        </Transition>
    }
}
