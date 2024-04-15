use super::super::{
    models::{BlogPost, UpdateBlogPostResult},
    server_fns::update_blog_post::UpdateBlogPost,
};
use leptos::*;
use leptos_router::{ActionForm, A};

#[component]
pub fn EditBlogPostForm(post: BlogPost) -> impl IntoView {
    let update_post_action = create_server_action::<UpdateBlogPost>();

    let result = move || {
        update_post_action.value().get().map(|res| match res {
            Ok(UpdateBlogPostResult { saved_time, .. }) => {
                view! { <span>{saved_time.format("Saved %d/%m/%Y %H:%M:%S").to_string()}</span> }
                    .into_view()
            }
            Err(err) => view! { <span>{format!("{err:?}")}</span> }.into_view(),
        })
    };

    let (href, set_href) = create_signal(format!("/blog/{}", post.slug));

    create_effect(move |_| {
        if let Some(Ok(UpdateBlogPostResult { slug, .. })) = update_post_action.value().get() {
            set_href(format!("/blog/{}", slug));
        }
    });

    view! {
        <ActionForm action=update_post_action class="flex flex-col">
            <input type="hidden" value=post.id.to_string() name="post[id]"/>
            <label class="form-control">
                <div class="label">
                    <span class="label-text">"Title"</span>
                </div>
                <input class="input input-bordered" value=&post.title name="post[title]"/>
            </label>
            <label class="form-control">
                <div class="label">
                    <span class="label-text">"Slug"</span>
                </div>
                <input class="input input-bordered" value=&post.slug name="post[slug]"/>
            </label>
            <label class="form-control">
                <div class="label">
                    <span class="label-text">"Slug"</span>
                </div>
                <textarea
                    style="height: 530px;"
                    class="textarea textarea-bordered font-mono"
                    name="post[md_content]"
                >
                    {post.md_content}
                </textarea>
            </label>
            <div class="form-control">
                <label class="label cursor-pointer">
                    <span class="label-text">"Published"</span>
                    <input
                        class="checkbox"
                        type="checkbox"
                        checked=post.published
                        name="post[published]"
                    />
                </label>
            </div>
            <button class="btn" type="submit">
                "Submit"
            </button>
            <A href=href>"Back to post"</A>
            {result}
        </ActionForm>
    }
}
