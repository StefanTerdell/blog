use super::{
    super::server_fns::get_blog_post_or_create_new::get_blog_post_or_create_new,
    BlogPostFiles::BlogPostFiles, EditBlogPostForm::EditBlogPostForm,
};
use leptos::*;
use leptos_router::use_params_map;

#[component]
pub fn EditBlogPost() -> impl IntoView {
    let params = use_params_map();
    let post = create_resource(
        move || params().get("slug").cloned(),
        |slug| async move { get_blog_post_or_create_new(slug.unwrap_or_default()).await },
    );

    view! {
        <Suspense fallback=move || {
            view! { <div class="loading loading-spinner mx-auto"></div> }
        }>
            {move || {
                match post.get() {
                    Some(Ok(post)) => {
                        view! {
                            <EditBlogPostForm post=post.clone() />
                            <BlogPostFiles blog_post_id=post.id />
                        }
                            .into_view()
                    }
                    Some(Err(err)) => format!("{err:?}").into_view(),
                    None => {
                        view! { <div class="loading loading-spinner mx-auto"></div> }.into_view()
                    }
                }
            }}

        </Suspense>
    }
}
