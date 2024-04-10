use chrono::{DateTime, Utc};
use leptos::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn Posts() -> impl IntoView {
    use crate::components::{github::IsAdmin, links::FA};
    use leptos_router::A;

    let articles = create_blocking_resource(|| (), move |_| get_posts());

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

#[component]
pub fn Post() -> impl IntoView {
    use leptos_router::use_params_map;

    let params = use_params_map();
    let slug = move || params.with(|params| params.get("slug").cloned().unwrap_or_default());
    let post = create_blocking_resource(move || slug(), |slug| async move { get_post(slug).await });

    view! {
        <Transition>
            {move || {
                match post.get() {
                    Some(Ok(Some(post))) => view! { <RenderPost post=post/> }.into_view(),
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

#[component]
fn RenderPost(post: PostData) -> impl IntoView {
    use crate::components::{github::IsAdmin, links::FA};
    use leptos_router::A;

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

#[component]
pub fn EditPost() -> impl IntoView {
    use leptos_router::use_params_map;

    let params = use_params_map();
    let slug = move || params.with(|params| params.get("slug").cloned().unwrap_or_default());
    let post = create_blocking_resource(move || slug(), move |slug| get_post_or_new(slug));

    view! {
        <Transition>
            {move || {
                match post.get() {
                    Some(Ok(post)) => view! { <EditPostForm post=post/> }.into_view(),
                    Some(Err(err)) => format!("{err:?}").into_view(),
                    None => {
                        view! { <div class="loading loading-spinner mx-auto"></div> }.into_view()
                    }
                }
            }}

        </Transition>
    }
}

#[component]
fn EditPostForm(post: PostData) -> impl IntoView {
    use leptos_router::{ActionForm, A};

    let action = create_server_action::<UpdatePost>();

    let result = move || {
        action.value().get().map(|res| match res {
            Ok(UpdatePostResult { saved_time, .. }) => {
                view! { <span>{saved_time.format("Saved %d/%m/%Y %H:%M:%S").to_string()}</span> }
                    .into_view()
            }
            Err(err) => view! { <span>{format!("{err:?}")}</span> }.into_view(),
        })
    };

    let (href, set_href) = create_signal(format!("/blog/{}", post.slug));

    create_effect(move |_| {
        if let Some(Ok(UpdatePostResult { slug, .. })) = action.value().get() {
            set_href(format!("/blog/{}", slug));
        }
    });

    view! {
        <ActionForm action=action class="flex flex-col">
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
                    style="height: 575px;"
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

#[derive(Serialize, Deserialize, Clone)]
pub struct PostListItem {
    slug: String,
    title: String,
    views: i64,
    published_time: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostData {
    id: i32,
    slug: String,
    title: String,
    views: i32,
    md_content: String,
    html_content: String,
    published: bool,
    published_time: DateTime<Utc>,
    edited_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdatePostData {
    id: i32,
    slug: String,
    title: String,
    md_content: String,
    published: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdatePostResult {
    saved_time: DateTime<Utc>,
    slug: String,
}

#[server]
async fn update_post(post: UpdatePostData) -> Result<UpdatePostResult, ServerFnError> {
    use crate::utils::user::ssr::AuthSession;
    use sqlx;

    if !expect_context::<AuthSession>()
        .current_user
        .is_some_and(|u| u.admin)
    {
        return Err(ServerFnError::new("Unauthorized"));
    };

    let published = post.published.is_some_and(|p| p == "on");
    let pool = expect_context::<sqlx::PgPool>();
    let html_content = markdown_to_html(&post.md_content);

    #[derive(Deserialize)]
    struct Status {
        published: bool,
        published_time: DateTime<Utc>,
    }

    let status = sqlx::query_as!(
        Status,
        "
            SELECT published, published_time
            FROM blog_posts
            WHERE id = $1
        ",
        &post.id
    )
    .fetch_one(&pool)
    .await?;

    let now = chrono::offset::Utc::now();

    let (published_time, edited_time) = if !status.published && published {
        (now.clone(), None)
    } else {
        (status.published_time, Some(now.clone()))
    };

    #[derive(Deserialize)]
    struct Returning {
        slug: String,
    }

    let result = sqlx::query_as!(
        Returning,
        "
            UPDATE blog_posts 
            SET 
                slug = $2,
                title = $3,
                md_content = $4,
                html_content = $5,
                published = $6,
                published_time = $7,
                edited_time = $8
            WHERE id = $1
            RETURNING slug
        ",
        &post.id,
        &post.slug,
        &post.title,
        &post.md_content,
        html_content,
        published,
        published_time,
        edited_time
    )
    .fetch_one(&pool)
    .await?;

    Ok(UpdatePostResult {
        saved_time: now,
        slug: result.slug,
    })
}

#[server]
async fn get_post_or_new(slug: String) -> Result<PostData, ServerFnError> {
    use crate::utils::user::ssr::AuthSession;
    use sqlx;

    if !expect_context::<AuthSession>()
        .current_user
        .is_some_and(|u| u.admin)
    {
        return Err(ServerFnError::new("Unauthorized"));
    };

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
            INSERT INTO blog_posts (slug, title, md_content, html_content, published, views)
            VALUES ($1, '', '', '', FALSE, 0)
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
    use crate::utils::user::ssr::AuthSession;
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

    Ok(post)
}

#[server]
async fn get_posts() -> Result<Vec<PostListItem>, ServerFnError> {
    use crate::utils::user::ssr::AuthSession;
    use sqlx;

    let admin = expect_context::<AuthSession>()
        .current_user
        .is_some_and(|u| u.admin);

    let pool = expect_context::<sqlx::PgPool>();
    let posts = sqlx::query_as!(
        PostListItem,
        "
            SELECT 
                slug,
                title,
                views,
                published_time
            FROM blog_posts
            WHERE $1 OR published
            ORDER BY published_time DESC
        ",
        admin
    )
    .fetch_all(&pool)
    .await?;

    Ok(posts)
}
