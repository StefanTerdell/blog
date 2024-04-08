use leptos::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn Posts() -> impl IntoView {
    use leptos_router::A;
    let articles = create_blocking_resource(|| (), move |_| async { get_posts().await.unwrap() });

    view! {
        <Transition>
            <For each=move || articles().unwrap_or_default() key=|post| post.title.clone() let:post>
                <A href=format!("/blog/{}", post.slug)>{post.title.clone()}</A>
                <p>{post.views} " views"</p>
            </For>
        </Transition>
    }
}

#[component]
pub fn Post() -> impl IntoView {
    use leptos_router::use_params_map;
    let params = use_params_map();
    let slug = move || params.with(|params| params.get("slug").cloned().unwrap_or_default());

    let post = create_blocking_resource(
        move || slug(),
        |slug| async move { get_post(slug).await.unwrap() },
    );

    view! {
        <Transition>
            {move || match post.get() {
                Some(post) => {
                    view! {
                        <>
                            <h1>{post.title}</h1>
                            <div class="markdown" inner_html=post.content></div>
                        </>
                    }
                }
                None => view! { <>"derp"</> },
            }}

        </Transition>
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PostListItem {
    slug: String,
    title: String,
    views: i64,
    published: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Post {
    title: String,
    content: String,
}

#[server]
async fn get_post(slug: String) -> Result<Post, ServerFnError> {
    use crate::user::ssr::AuthSession;
    use pulldown_cmark::{html, Options, Parser};
    use sqlx;

    let admin = expect_context::<AuthSession>()
        .current_user
        .is_some_and(|u| u.admin);
    let pool = expect_context::<sqlx::PgPool>();
    let mut post = sqlx::query_as!(
        Post,
        "
            UPDATE blog_posts
            SET views = views + 1
            WHERE slug = $1 AND $2 OR published
            RETURNING title, content
        ",
        slug,
        admin
    )
    .fetch_one(&pool)
    .await?;

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    // options.insert(Options::DIN_MAMMA);
    // options.insert(Options:: );

    let parser = Parser::new_ext(&post.content, options);
    let mut html = String::new();
    html::push_html(&mut html, parser);

    post.content = html;

    Ok(post)
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
