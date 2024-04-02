use leptos::*;

#[component]
pub fn Provider(children: ChildrenFn) -> impl IntoView {
    use crate::user::User;

    let user_signal = create_rw_signal(None::<User>);
    let user_resource = create_resource(|| (), move |_| get_user_from_session());

    create_effect(move |_| {
        if let Some(Ok(response)) = user_resource.get() {
            user_signal.set(response);
        }
    });

    provide_context(user_signal);

    view! { <>{children()}</> }
}

#[component]
pub fn AccountButton() -> impl IntoView {
    view! {
        <LoggedIn fallback=move || view! { <LogInButton/> }>
            <LogOutButton/>
        </LoggedIn>
    }
}

#[component]
pub fn LoggedIn(#[prop(optional, into)] fallback: ViewFn, children: ChildrenFn) -> impl IntoView {
    use crate::user::User;

    let user = expect_context::<RwSignal<Option<User>>>();

    view! {
        <Show when=move || user().is_some() fallback=move || { fallback.run() }>
            {children()}
        </Show>
    }
}

#[component]
pub fn LogInButton(#[prop(optional, into)] class: String) -> impl IntoView {
    use leptos_router::ActionForm;
    let action = create_server_action::<LogIn>();

    view! {
        <ActionForm action=action>
            <input class=class type="submit" value="Log in with GitHub"/>
        </ActionForm>
    }
}

#[component]
pub fn LogOutButton(#[prop(optional, into)] class: String) -> impl IntoView {
    use crate::user::User;
    use leptos_router::ActionForm;

    let action = create_server_action::<LogOut>();
    let user = expect_context::<RwSignal<Option<User>>>();

    create_effect(move |_| {
        if action.value().get().is_some_and(|res| res.is_ok()) {
            user.set(None);
        }
    });

    view! {
        <ActionForm action=action>
            <input class=class type="submit" value="Log out from GitHub"/>
        </ActionForm>
    }
}

#[component]
pub fn Callback() -> impl IntoView {
    use crate::user::User;
    use leptos::logging::log;
    use leptos_router::{use_navigate, use_query, NavigateOptions, Params};

    #[derive(Debug, Params, Clone, PartialEq)]
    pub struct OAuthParams {
        pub code: Option<String>,
        pub state: Option<String>,
    }

    let exchange_code_action = create_server_action::<ExchangeCode>();
    let navigate = use_navigate();
    let query = use_query::<OAuthParams>();
    let user = expect_context::<RwSignal<Option<User>>>();
    let (error, set_error) = create_signal(None::<String>);

    create_effect(move |_| {
        log!("In response effect");

        if let Some(response) = exchange_code_action.value().get() {
            match response {
                Ok(response) => {
                    log!("Received response {response:?}");
                    navigate("/guestbook", NavigateOptions::default());

                    user.set(Some(response))
                }
                Err(err) => {
                    set_error(Some("Recieved an error from the server :(".to_string()));

                    log!("Received error {err:?}");
                }
            }
        }
    });

    create_effect(move |_| {
        log!("In query effect, {:?}", query.get_untracked());

        if let Ok(OAuthParams { code, state }) = query.get_untracked() {
            if let (Some(state), Some(code)) = (state, code) {
                exchange_code_action.dispatch(ExchangeCode {
                    provided_csrf: state,
                    code,
                });
            } else {
                log!("Missing state or code in query")
            }
        } else {
            log!("Unable to parse query")
        }
    });

    view! {
        <Show
            when=move || error().is_none()
            fallback=move || {
                view! { <p>{error()}</p> }
            }
        >

            <p class="animate-pulse">"Doing auth stuff, hold up a minute"</p>
        </Show>
    }
}

#[server]
async fn exchange_code(
    provided_csrf: String,
    code: String,
) -> Result<crate::user::User, ServerFnError> {
    use crate::user::{ssr::AuthSession, User};
    use leptos::logging::log;
    use oauth2::{
        basic::BasicClient, reqwest::async_http_client, AuthorizationCode, TokenResponse,
    };
    use serde::Deserialize;
    use sqlx::PgPool;

    log!("Exchange request started, {provided_csrf:?} {code:?}");

    let pool = expect_context::<PgPool>();
    let oauth_client = expect_context::<BasicClient>();
    let auth_session = expect_context::<AuthSession>();

    let csrf_found = sqlx::query!(
        "DELETE FROM csrf_token WHERE csrf_token = $1 RETURNING csrf_token",
        &provided_csrf
    )
    .fetch_optional(&pool)
    .await?
    .is_some();

    if !csrf_found {
        return Err(ServerFnError::new("No matching CSRF token found"));
    }

    let token_response = oauth_client
        .exchange_code(AuthorizationCode::new(code.clone()))
        .request_async(async_http_client)
        .await?;

    let access_token = token_response.access_token().secret();

    log!("Token received, getting user info");

    #[derive(Deserialize, Debug)]
    struct GithubUser {
        id: i32,
        html_url: String,
        // name: String,
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
        .await
        .unwrap();

    let user = match User::get_by_email(&github_user.html_url, &pool)
        .await
        .map_err(|err| ServerFnError::new(err.to_string()))?
    {
        Some(user) => user,
        None => User::register(&github_user.id, &github_user.html_url, &pool)
            .await
            .map_err(|err| ServerFnError::new(err.to_string()))?,
    };

    log!("User info received {user:?}, updating stores");

    auth_session.login_user(user.id);

    sqlx::query!("DELETE FROM github_token WHERE user_id = $1", &user.id)
        .execute(&pool)
        .await?;

    sqlx::query!(
        "INSERT INTO github_token (user_id, access_token, refresh_token, created_at) VALUES ($1, $2, $3, EXTRACT(epoch FROM NOW())::BIGINT)",
        &user.id ,
        &access_token,
        "NO REFRESH TOKEN"
    )
    .execute(&pool)
    .await?;

    Ok(user)
}

#[server]
async fn log_out() -> Result<(), ServerFnError> {
    use crate::user::ssr::AuthSession;
    use serde::Serialize;
    use sqlx::PgPool;

    let auth_session = expect_context::<AuthSession>();
    let pool = expect_context::<PgPool>();

    if let Some(user) = &auth_session.current_user {
        let result = sqlx::query!(
            "DELETE FROM github_token WHERE user_id = $1 RETURNING access_token",
            user.id
        )
        .fetch_optional(&pool)
        .await?;

        if let Some(access_token) = result.map(|row| row.access_token) {
            #[derive(Serialize)]
            struct Payload {
                access_token: String,
            }

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
                .json(&Payload { access_token })
                .send()
                .await?;
        }

        auth_session.logout_user();
    };

    Ok(())
}

#[server]
async fn log_in() -> Result<(), ServerFnError> {
    use leptos::logging::log;
    use oauth2::{CsrfToken, Scope};
    use sqlx;

    let oauth_client = expect_context::<oauth2::basic::BasicClient>();
    let pool = expect_context::<sqlx::PgPool>();

    let (url, csrf_token) = oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("read:user".to_string()))
        .url();

    sqlx::query!(
        "INSERT INTO csrf_token (csrf_token) VALUES ($1)",
        csrf_token.secret()
    )
    .execute(&pool)
    .await?;

    let url = url.to_string();

    log!("Redirecting to {}", url);

    leptos_axum::redirect(&url);

    Ok(())
}

#[server]
pub async fn get_user_from_session() -> Result<Option<crate::user::User>, ServerFnError> {
    use crate::user::ssr::AuthSession;

    let auth_session = expect_context::<AuthSession>();

    Ok(auth_session.current_user)
}
