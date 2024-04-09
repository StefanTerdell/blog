use crate::user::User;
use icondata::AiGithubOutlined;
use leptos::*;
use leptos_icons::*;

pub type UserResource = Resource<(), Result<Option<User>, ServerFnError>>;

#[component]
pub fn Provider(children: ChildrenFn) -> impl IntoView {
    let user_resource = create_blocking_resource(|| (), move |_| get_user_from_session());

    provide_context(user_resource);

    view! { <Transition>{children()}</Transition> }
}

#[component]
pub fn LoggedIn(#[prop(optional, into)] fallback: ViewFn, children: ChildrenFn) -> impl IntoView {
    let user = expect_context::<UserResource>();

    view! {
        <Show when=move || matches!(user(), Some(Ok(Some(_)))) fallback=move || { fallback.run() }>
            {children()}
        </Show>
    }
}

#[component]
pub fn IsAdmin(#[prop(optional, into)] fallback: ViewFn, children: ChildrenFn) -> impl IntoView {
    let user = expect_context::<UserResource>();

    view! {
        <Show
            when=move || matches!(user(), Some(Ok(Some(User { admin: true, .. }))))
            fallback=move || { fallback.run() }
        >
            {children()}
        </Show>
    }
}

#[component]
pub fn AccountButton(
    #[prop(optional)] small: bool,
    #[prop(optional)] neutral: bool,
) -> impl IntoView {
    view! {
        <LoggedIn fallback=move || view! { <LogInButton small=small neutral=neutral/> }>
            <LogOutButton small=small neutral=neutral/>
        </LoggedIn>
    }
}

#[component]
pub fn LogInButton(
    #[prop(optional)] small: bool,
    #[prop(optional)] neutral: bool,
) -> impl IntoView {
    use leptos_router::{use_location, ActionForm};

    let location = use_location();
    let action = create_server_action::<LogIn>();
    let user = expect_context::<UserResource>();
    let text = if small {
        "Log in"
    } else {
        "Log in with GitHub"
    };

    view! {
        <ActionForm action=action>
            <input type="hidden" name="redirect_to" value=move || location.pathname/>
            <GithubIconButton text=text loading=user.loading() small=small neutral=neutral/>
        </ActionForm>
    }
}

#[component]
pub fn LogOutButton(
    #[prop(optional)] small: bool,
    #[prop(optional)] neutral: bool,
) -> impl IntoView {
    use leptos_router::ActionForm;

    let action = create_server_action::<LogOut>();
    let user = expect_context::<UserResource>();
    let text = if small {
        "Log out"
    } else {
        "Log out from GitHub"
    };

    create_effect(move |_| {
        if let Some(res) = action.value().get() {
            user.set(res.map(|_| None));
        }
    });

    view! {
        <ActionForm action=action>
            <GithubIconButton text=text loading=user.loading() small=small neutral=neutral/>
        </ActionForm>
    }
}

#[component]
fn GithubIconButton(
    text: &'static str,
    loading: Signal<bool>,
    small: bool,
    neutral: bool,
) -> impl IntoView {
    let mut class = "btn btn-sm".to_string();

    if neutral {
        class = format!("{class} btn-neutral");
    }

    if small {
        class = format!("{class} w-20");
    } else {
        class = format!("{class} w-48");
    }

    view! {
        <button class=class class:disabled=loading type="submit" disabled=move || loading()>
            <div class="flex gap-2 items-center">
                <span>{text}</span>
                <Show when=move || !small>
                    <Show
                        when=move || loading()
                        fallback=move || {
                            view! { <Icon width="24" height="24" icon=AiGithubOutlined/> }
                        }
                    >

                        <div class="w-6 h-6 loading-spinner"></div>
                    </Show>
                </Show>
            </div>
        </button>
    }
}

#[server]
async fn log_in(redirect_to: String) -> Result<(), ServerFnError> {
    use oauth2::{CsrfToken, Scope};
    use sqlx;

    let oauth_client = expect_context::<oauth2::basic::BasicClient>();
    let pool = expect_context::<sqlx::PgPool>();

    let (url, csrf_token) = oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("read:user".to_string()))
        .url();

    sqlx::query!(
        "INSERT INTO csrf_tokens (csrf_token, redirect_to) VALUES ($1, $2)",
        csrf_token.secret(),
        redirect_to
    )
    .execute(&pool)
    .await?;

    let url = url.to_string();

    leptos_axum::redirect(&url);

    Ok(())
}

#[server]
async fn log_out() -> Result<(), ServerFnError> {
    use crate::user::ssr::AuthSession;
    use sqlx::PgPool;

    let auth_session = expect_context::<AuthSession>();
    let pool = expect_context::<PgPool>();

    if let Some(user) = &auth_session.current_user {
        delete_token(&user.id, &pool).await?;

        auth_session.logout_user();
    };

    Ok(())
}

#[server]
pub async fn get_user_from_session() -> Result<Option<User>, ServerFnError> {
    use crate::user::ssr::AuthSession;
    let auth_session = expect_context::<AuthSession>();

    Ok(auth_session.current_user)
}

#[cfg(feature = "ssr")]
pub async fn exchange_code(
    provided_csrf: String,
    code: String,
    pool: sqlx::PgPool,
    oauth_client: oauth2::basic::BasicClient,
    auth_session: crate::user::ssr::AuthSession,
) -> anyhow::Result<String> {
    use anyhow::bail;
    use oauth2::{reqwest::async_http_client, AuthorizationCode, TokenResponse};
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct CsrfRow {
        redirect_to: String,
    }

    let csrf_row = sqlx::query_as!(
        CsrfRow,
        "DELETE FROM csrf_tokens WHERE csrf_token = $1 RETURNING redirect_to",
        &provided_csrf
    )
    .fetch_optional(&pool)
    .await?;

    let Some(CsrfRow { redirect_to }) = csrf_row else {
        bail!("No matching CSRF token found");
    };

    let token_response = oauth_client
        .exchange_code(AuthorizationCode::new(code.clone()))
        .request_async(async_http_client)
        .await?;

    let access_token = token_response.access_token().secret();

    #[derive(Deserialize, Debug)]
    struct GithubUser {
        id: i32,
        html_url: String,
        name: Option<String>,
        login: String,
    }

    let github_user = reqwest::Client::new()
        .get("https://api.github.com/user")
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "guestbook")
        .bearer_auth(&access_token)
        .send()
        .await?
        .json::<GithubUser>()
        .await?;

    let user = match User::get_by_id(github_user.id, &pool).await? {
        Some(user) => user,
        None => {
            User::register(
                &github_user.id,
                &github_user.name.unwrap_or(github_user.login),
                &github_user.html_url,
                &pool,
            )
            .await?
        }
    };

    delete_token(&user.id, &pool)
        .await
        .map_err(|err| anyhow::anyhow!("{err}"))?;

    auth_session.login_user(user.id);

    drop(auth_session);

    sqlx::query!(
        "INSERT INTO github_tokens (user_id, access_token) VALUES ($1, $2)",
        &user.id,
        &access_token,
    )
    .execute(&pool)
    .await?;

    Ok(redirect_to)
}

#[cfg(feature = "ssr")]
async fn delete_token(user_id: &i32, pool: &sqlx::PgPool) -> Result<(), ServerFnError> {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct Payload {
        access_token: String,
    }

    let result = sqlx::query_as!(
        Payload,
        "DELETE FROM github_tokens WHERE user_id = $1 RETURNING access_token",
        user_id
    )
    .fetch_optional(pool)
    .await?;

    if let Some(payload) = result {
        reqwest::Client::new()
            .delete(format!(
                "https://api.github.com/applications/{}/token",
                std::env::var("GITHUB_CLIENT_SECRET").unwrap()
            ))
            .header("Accept", "application/vnd.github+json")
            .basic_auth(
                std::env::var("GITHUB_CLIENT_ID").unwrap().to_string(),
                Some(std::env::var("GITHUB_CLIENT_SECRET").unwrap().to_string()),
            )
            .header("X-GitHub-Api-Version", "2022-11-28")
            .json(&payload)
            .send()
            .await?;
    }

    Ok(())
}
