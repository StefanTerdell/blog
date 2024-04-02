use leptos::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn Guestbook() -> impl IntoView {
    use crate::github::{AccountButton, LoggedIn};
    view! {
        <div class="flex flex-col gap-2">
            <AccountButton/>
            <LoggedIn>
                <NewPost/>
            </LoggedIn>
            <Await future=move || async { get_guestbook_posts().await.unwrap() } let:posts>
                <Posts posts=posts.clone()/>
            </Await>
        </div>
    }
}

#[component]
fn NewPost() -> impl IntoView {
    use leptos_router::ActionForm;
    let create_post = create_server_action::<CreateGuestbookPost>();

    view! {
        <ActionForm action=create_post>
            <label>"Say something nice" <input type="text" min=3 name="content"/></label>
            <input type="submit" value="Submit"/>
        </ActionForm>
    }
}

#[component]
fn Posts(posts: Vec<GuestbookPost>) -> impl IntoView {
    let posts = posts
        .into_iter()
        .filter_map(|post| {
            if post.published {
                Some(view! { <Post post=post/> })
            } else {
                None
            }
        })
        .collect_view();

    view! { <>{posts}</> }
}

#[component]
fn Post(post: GuestbookPost) -> impl IntoView {
    view! {
        <div>
            <strong>{post.user_name} ": "</strong>
            <span>{post.content}</span>
        </div>
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuestbookPost {
    pub id: i32,
    pub user_id: String,
    pub user_name: String,
    pub user_url: String,
    pub content: String,
    pub published: bool,
    pub published_timestamp: i64,
    pub edited_timestamp: Option<i64>,
}

#[server]
async fn get_guestbook_posts() -> Result<Vec<GuestbookPost>, ServerFnError> {
    use sqlx;

    let pool = expect_context::<sqlx::PgPool>();
    let posts = sqlx::query_as!(GuestbookPost, "SELECT * FROM guestbook_post")
        .fetch_all(&pool)
        .await?;

    Ok(posts)
}

#[server]
async fn create_guestbook_post(content: String) -> Result<GuestbookPost, ServerFnError> {
    use sqlx;

    let pool = expect_context::<sqlx::PgPool>();

    let post = sqlx::query_as!(
        GuestbookPost,
        "
        INSERT INTO guestbook_post (
            user_id,
            user_name,
            user_url,
            content,
            published,
            published_timestamp
        ) VALUES (
            $1,
            $2,
            $3,
            $4,
            FALSE,
            EXTRACT(epoch FROM NOW())::BIGINT
        ) RETURNING 
            id,
            user_id,
            user_name,
            user_url,
            content,
            published,
            published_timestamp,
            edited_timestamp
        ",
        "The user id".into(),
        "The user name".into(),
        "The user url".into(),
        content
    )
    .fetch_one(&pool)
    .await?;

    Ok(post)
}
