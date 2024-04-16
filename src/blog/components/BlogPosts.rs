use super::super::server_fns::get_blog_post_list::get_blog_post_list;
use crate::{components::links::FA, github::components::IsAdmin};
use leptos::*;
use leptos_router::A;

#[component]
pub fn BlogPosts() -> impl IntoView {
    let articles = create_blocking_resource(|| (), move |_| get_blog_post_list());

    view! {
        <Transition>
            <div class="flex flex-col gap-4">
                {move || match articles() {
                    Some(Ok(articles)) => {
                        articles
                            .into_iter()
                            .map(|post| {
                                view! {
                                    <div>
                                        <FA class="text-2xl font-serif" href=post.slug>
                                            {post.title}
                                        </FA>
                                        <p class="prose-neutral italic font-mono">
                                            <time>
                                                {post.published_time.format("%d/%m/%Y").to_string()}
                                            </time>
                                            <span>" - " {post.views} " views"</span>
                                        </p>
                                    </div>
                                }
                            })
                            .collect_view()
                    }
                    Some(Err(err)) => format!("{err:?}").into_view(),
                    None => {
                        view! { <div class="loading loading-spinner mx-auto"></div> }.into_view()
                    }
                }}

            </div>
        </Transition>
        <IsAdmin>
            <A class="link" href="new-post/edit">
                "New post"
            </A>
        </IsAdmin>
    }
}
