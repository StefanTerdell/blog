use super::super::models::BlogPost;
use crate::{components::links::FA, github::components::IsAdmin};
use leptos::*;
use leptos_router::A;

#[component]
pub fn RenderBlogPost(post: BlogPost) -> impl IntoView {
    view! {
        <article class="prose max-w-6xl">
            <h1 class="text-5xl mb-2 font-serif">
                <FA href="/blog">{post.title}</FA>
            </h1>
            <time class="italic font-mono">
                {post.published_time.format("Published %d/%m/%Y").to_string()}
                {post.edited_time.map(|e| e.format(", Edited %d/%m/%Y").to_string())}
            </time>
            <div class="markdown" inner_html=post.html_content></div>
        </article>
        <IsAdmin>
            <A class="link" href="edit">
                "Edit"
            </A>
        </IsAdmin>
    }
}
