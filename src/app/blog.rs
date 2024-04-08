use chrono::{DateTime, Utc};
use leptos::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn Posts() -> impl IntoView {
    use crate::components::FA;
    use leptos_router::A;

    let articles = create_blocking_resource(|| (), move |_| async { get_posts().await.unwrap() });

    view! {
        <Transition>
            <For each=move || articles().unwrap_or_default() key=|post| post.title.clone() let:post>
                <FA href=format!("/blog/{}", post.slug)>{post.title.clone()}</FA>
                <p>{post.views} " views"</p>
            </For>
        </Transition>
        <A class="link" href="new-post/edit">
            "New"
        </A>
    }
}

#[component]
pub fn Post() -> impl IntoView {
    // use crate::github::LoggedIn;
    use leptos_router::use_params_map;

    let params = use_params_map();
    let slug = move || params.with(|params| params.get("slug").cloned().unwrap_or_default());
    let post = create_blocking_resource(move || slug(), |slug| async move { get_post(slug).await });

    view! {
        <Transition>
            {move || {
                post.get()
                    .map(|post| match post {
                        Ok(Some(post)) => view! { <RenderPost post=post/> }.into_view(),
                        Ok(None) => "No post found".into_view(),
                        Err(err) => format!("{err:?}").into_view(),
                    })
            }}

        </Transition>
    }
}

#[component]
fn RenderPost(post: PostData) -> impl IntoView {
    use leptos_router::A;

    view! {
        <h1>{post.title}</h1>
        <time>"Published " {post.published_time.format("%d/%m/%Y %H:%M").to_string()}</time>
        <div class="markdown" inner_html=post.content></div>
        <A class="link" href="edit">
            "Edit"
        </A>
    }
}

// <LoggedIn>
// </LoggedIn>

#[component]
pub fn EditPost() -> impl IntoView {
    use leptos_router::use_params_map;

    let params = use_params_map();
    let slug = move || params.with(|params| params.get("slug").cloned().unwrap_or_default());
    let post = create_blocking_resource(
        move || slug(),
        |slug| async move { get_post_or_new(slug).await },
    );

    view! {
        <Transition>
            {move || {
                post.get()
                    .map(|post| match post {
                        Ok(post) => view! { <EditPostForm post=post/> }.into_view(),
                        Err(err) => format!("{err:?}").into_view(),
                    })
            }}

        </Transition>
    }
}

#[component]
// fn EditPostForm<F: Fn() + 'static>(post: PostData, refetch: F) -> impl IntoView {
fn EditPostForm(post: PostData) -> impl IntoView {
    use leptos_router::{ActionForm, A};

    let action = create_server_action::<UpdatePost>();

    let result = move || {
        action.value().get().map(|res| match res {
            Ok(_) => view! { <span>"Ok!"</span> }.into_view(),
            Err(err) => view! { <span>{format!("{err:?}")}</span> }.into_view(),
        })
    };

    view! {
        <ActionForm action=action>
            <input type="hidden" value=post.id.to_string() name="post[id]"/>
            <input class="input" value=&post.title name="post[title]"/>
            <input class="input" value=&post.slug name="post[slug]"/>
            <textarea class="input" name="post[content]">
                {post.content}
            </textarea>
            <input class="checkbox" type="checkbox" checked=post.published name="post[published]"/>
            <button class="btn" type="submit">
                "Submit"
            </button>
            <A href=format!("/blog/{}", post.slug)>"Back to post"</A>
            {result}
        </ActionForm>
    }
}
// <input type="checkbox" checked=post.published name="post[published]"/>

#[derive(Serialize, Deserialize, Clone)]
pub struct PostListItem {
    slug: String,
    title: String,
    views: i64,
    published: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostData {
    id: i32,
    slug: String,
    title: String,
    views: i32,
    content: String,
    published: bool,
    published_time: DateTime<Utc>,
    edited_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdatePostData {
    id: i32,
    slug: String,
    title: String,
    content: String,
    published: Option<String>,
}

#[server]
async fn update_post(post: UpdatePostData) -> Result<(), ServerFnError> {
    use sqlx;
    let pool = expect_context::<sqlx::PgPool>();
    sqlx::query!(
        "UPDATE blog_posts SET slug = $2, title = $3, content = $4, published = $5 WHERE id = $1",
        &post.id,
        &post.slug,
        &post.title,
        &post.content,
        &post.published.is_some_and(|text| text == "on")
    )
    .execute(&pool)
    .await?;

    Ok(())
}

#[server]
async fn get_post_or_new(slug: String) -> Result<PostData, ServerFnError> {
    use sqlx;
    let pool = expect_context::<sqlx::PgPool>();
    let post = sqlx::query_as!(PostData, "SELECT * FROM blog_posts WHERE slug = $1", &slug)
        .fetch_optional(&pool)
        .await?;

    if let Some(post) = post {
        return Ok(post);
    }

    let post = sqlx::query_as!(
        PostData,
        "
            INSERT INTO blog_posts (slug, title, content, published, views)
            VALUES ($1, '', '', FALSE, 0)
            RETURNING *
        ",
        &slug
    )
    .fetch_one(&pool)
    .await?;

    Ok(post)
}

#[cfg(feature = "ssr")]
pub fn markdown_to_html(content: &String) -> String {
    use pulldown_cmark::{html, Options, Parser};

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(content, options);
    let mut html = String::new();
    html::push_html(&mut html, parser);

    html
}

#[server]
async fn get_post(slug: String) -> Result<Option<PostData>, ServerFnError> {
    use crate::user::ssr::AuthSession;
    use sqlx;

    let admin = expect_context::<AuthSession>()
        .current_user
        .is_some_and(|u| u.admin);
    let pool = expect_context::<sqlx::PgPool>();
    let post = sqlx::query_as!(
        PostData,
        "
            UPDATE blog_posts
            SET views = views + 1
            WHERE slug = $1 AND ($2 OR published)
            RETURNING *
        ",
        slug,
        admin
    )
    .fetch_optional(&pool)
    .await?;

    // if let Some(mut post) = post {
    // };

    Ok(post.map(|mut post| {
        post.content = markdown_to_html(&post.content);
        post
    }))
}

#[server]
async fn get_posts() -> Result<Vec<PostListItem>, ServerFnError> {
    use crate::user::ssr::AuthSession;
    use sqlx;

    let user = expect_context::<AuthSession>().current_user;
    let pool = expect_context::<sqlx::PgPool>();
    let posts = sqlx::query_as!(
        PostListItem,
        "
            SELECT 
                slug,
                title,
                views,
                published
            FROM blog_posts
            WHERE $1 OR published
        ",
        user.as_ref().is_some_and(|u| u.admin),
    )
    .fetch_all(&pool)
    .await?;

    Ok(posts)
}