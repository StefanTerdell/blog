use chrono::{DateTime, Utc};
use leptos::*;
use serde::{Deserialize, Serialize};

use crate::user::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuestbookPost {
    pub id: i32,
    pub user_id: i32,
    pub user_name: String,
    pub user_url: String,
    pub content: String,
    pub published: bool,
    pub created_time: DateTime<Utc>,
}

#[component]
pub fn Guestbook() -> impl IntoView {
    use crate::github::{LogInButton, LoggedIn, UserResource};

    let user = expect_context::<UserResource>();
    let posts = create_blocking_resource(
        move || user(),
        |_| async move { get_guestbook_posts().await.unwrap_or_default() },
    );

    view! {
        <div class="flex flex-col gap-2">
            <LoggedIn fallback=move || view! { <LogInButton/> }>
                <NewPost refetch_posts=move || posts.refetch()/>
            </LoggedIn>
            <Transition>
                <For
                    each=move || posts().unwrap_or_default()
                    key=|post| (post.id, post.published)
                    let:post
                >
                    <Post post=post refetch_posts=move || posts.refetch()/>
                </For>
            </Transition>
        </div>
    }
}

#[component]
fn NewPost<F: Fn() + 'static>(refetch_posts: F) -> impl IntoView {
    use crate::github::LogOutButton;
    use leptos_router::ActionForm;

    let create_post = create_server_action::<CreateGuestbookPost>();

    create_effect(move |_| {
        if create_post.value().get().is_some() {
            refetch_posts();
        };
    });

    view! {
        <>
            {move || match create_post.value().get() {
                Some(Ok(_)) => view! { <i>"Thanks for posting in my guestbook!"</i> }.into_view(),
                Some(Err(_)) => {
                    view! { <p class="text-red-500">"Something went wrong :("</p> }.into_view()
                }
                None => {
                    view! {
                        <div class="flex mx-auto">
                            <ActionForm action=create_post>
                                <div class="join">
                                    <input
                                        placeholder="Say something nice :)"
                                        class="input input-bordered input-sm mr-2 join-item"
                                        disabled=move || create_post.pending().get()
                                        type="text"
                                        min=3
                                        name="content"
                                    />
                                    <input
                                        class="btn btn-sm mr-2 join-item"
                                        disabled=move || create_post.pending().get()
                                        type="submit"
                                        value="Submit"
                                    />
                                </div>

                            </ActionForm>
                            <LogOutButton small=true neutral=true/>
                        </div>
                    }
                        .into_view()
                }
            }}
        </>
    }
}

#[component]
fn Post<F: Fn() + 'static>(post: GuestbookPost, refetch_posts: F) -> impl IntoView {
    use crate::{components::Fa, github::UserResource};
    use leptos_router::ActionForm;

    let delete_action = create_server_action::<DeletePost>();
    let publish_action = create_server_action::<PublishPost>();
    let user = expect_context::<UserResource>();

    create_effect(move |_| {
        if delete_action.value().get().is_some() || publish_action.value().get().is_some() {
            refetch_posts();
        }
    });

    view! {
        <div>
            <Fa href=post.user_url>{post.user_name}</Fa>
            <span>": " {post.content}</span>
            <Show when=move || !post.published>
                <span class="badge">"This post is awaiting moderation."</span>
                <Show when=move || { matches!(user(), Some(Ok(Some(User { admin: true, .. })))) }>
                    <ActionForm action=publish_action>
                        <input type="hidden" name="post_id" value=post.id/>
                        <input type="submit" value="Publish"/>
                    </ActionForm>
                </Show>
            </Show>
            <Show when=move || {
                match user() {
                    Some(Ok(Some(User { admin, id, .. }))) => admin || id == post.user_id,
                    _ => false,
                }
            }>
                <ActionForm action=delete_action>
                    <input type="hidden" name="post_id" value=post.id/>
                    <input type="submit" value="Delete post"/>
                </ActionForm>
            </Show>
        </div>
    }
}

#[server]
async fn publish_post(post_id: i32) -> Result<(), ServerFnError> {
    use crate::user::ssr::AuthSession;
    use sqlx;

    let auth_session = expect_context::<AuthSession>();
    if !auth_session.current_user.is_some_and(|u| u.admin) {
        return Err(ServerFnError::new("Unauthorized"));
    };

    let pool = expect_context::<sqlx::PgPool>();

    sqlx::query!(
        "UPDATE guestbook_posts SET published=true WHERE id = $1",
        post_id
    )
    .execute(&pool)
    .await?;

    Ok(())
}

#[server]
async fn delete_post(post_id: i32) -> Result<(), ServerFnError> {
    use crate::user::ssr::AuthSession;
    use sqlx;

    let auth_session = expect_context::<AuthSession>();
    let Some(user) = auth_session.current_user else {
        return Err(ServerFnError::new("Unauthorized"));
    };

    let pool = expect_context::<sqlx::PgPool>();

    let _n = if user.admin {
        sqlx::query!("DELETE FROM guestbook_posts WHERE id = $1", post_id)
            .execute(&pool)
            .await?
    } else {
        sqlx::query!(
            "DELETE FROM guestbook_posts WHERE id = $1 AND user_id = $2",
            post_id,
            user.id
        )
        .execute(&pool)
        .await?
    };

    Ok(())
}

#[server]
async fn get_guestbook_posts() -> Result<Vec<GuestbookPost>, ServerFnError> {
    use crate::user::ssr::AuthSession;
    use sqlx;

    let user = expect_context::<AuthSession>().current_user;
    let pool = expect_context::<sqlx::PgPool>();
    let posts = sqlx::query_as!(
        GuestbookPost,
        "
            SELECT 
                guestbook_posts.*,
                github_users.name AS user_name,
                github_users.url AS user_url
            FROM guestbook_posts 
            JOIN github_users ON guestbook_posts.user_id = github_users.id
            WHERE $1 OR guestbook_posts.published=true OR guestbook_posts.user_id = $2
            ORDER BY guestbook_posts.created_time DESC
        ",
        user.as_ref().is_some_and(|u| u.admin),
        user.as_ref().map(|u| u.id).unwrap_or(-1)
    )
    .fetch_all(&pool)
    .await?;

    Ok(posts)
}

#[server]
async fn create_guestbook_post(content: String) -> Result<(), ServerFnError> {
    use crate::user::ssr::AuthSession;
    use sqlx;

    let auth_session = expect_context::<AuthSession>();
    let Some(user) = auth_session.current_user else {
        return Err(ServerFnError::new("Unauthorized"));
    };

    let pool = expect_context::<sqlx::PgPool>();

    sqlx::query!(
        "
            INSERT INTO guestbook_posts (
                user_id,
                content,
                published
            ) VALUES (
                $1,
                $2,
                FALSE
            )
        ",
        user.id,
        content
    )
    .execute(&pool)
    .await?;

    Ok(())
}
