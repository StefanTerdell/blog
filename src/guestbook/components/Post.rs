use super::super::{
    models,
    server_fns::{delete_post::DeletePost, publish_post::PublishPost},
};
use crate::{
    components::{github::UserResource, links::Fa},
    utils::user::User,
};
use leptos::*;
use leptos_router::ActionForm;

#[component]
pub fn Post<F: Fn() + 'static>(post: models::GuestbookPost, refetch_posts: F) -> impl IntoView {
    let delete_action = create_server_action::<DeletePost>();
    let publish_action = create_server_action::<PublishPost>();
    let user = expect_context::<UserResource>();

    create_effect(move |_| {
        if delete_action.value().get().is_some() || publish_action.value().get().is_some() {
            refetch_posts();
        }
    });

    view! {
        <div class="flex flex-col items-center">
            <Show when=move || !post.published>
                <i class="badge">"This post is awaiting moderation."</i>
            </Show>
            <div>
                <Fa href=post.user_url>{post.user_name}</Fa>
                <span>": " {post.content}</span>
            </div>
            <div class="flex gap-2">
                <Show when=move || {
                    !post.published && matches!(user(), Some(Ok(Some(User { admin: true, .. }))))
                }>
                    <ActionForm action=publish_action>
                        <input type="hidden" name="post_id" value=post.id/>
                        <input class="cursor-pointer" type="submit" value="Publish"/>
                    </ActionForm>
                </Show>
                <Show when=move || {
                    match user() {
                        Some(Ok(Some(User { admin, id, .. }))) => admin || id == post.user_id,
                        _ => false,
                    }
                }>
                    <ActionForm action=delete_action>
                        <input type="hidden" name="post_id" value=post.id/>
                        <input class="cursor-pointer" type="submit" value="Delete post"/>
                    </ActionForm>
                </Show>
            </div>
        </div>
    }
}
