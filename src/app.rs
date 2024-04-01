use crate::articles::ARTICLES;
use crate::error_template::{AppError, ErrorTemplate};
use icondata::{AiGithubFilled, AiLinkedinFilled};
use leptos::*;
use leptos_icons::*;
use leptos_meta::*;
use leptos_router::*;
mod guestbook;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let error_template = || {
        let mut outside_errors = Errors::default();
        outside_errors.insert_with_default_key(AppError::NotFound);
        view! { <ErrorTemplate outside_errors/> }.into_view()
    };

    view! {
        <Title text="Stefan tries to think"/>
        <Stylesheet id="leptos" href="/pkg/blog.css"/>
        <Router fallback=error_template>
            <Layout>
                <Routes>
                    <Route path="" view=Home/>
                    <Route path="work" view=Work/>
                    <Route path="blog" view=Blog/>
                    <Route ssr=SsrMode::Async path="guestbook" view=guestbook::Guestbook/>
                </Routes>
            </Layout>
        </Router>
    }
}

#[component]
fn Layout(children: Children) -> impl IntoView {
    view! {
        <main class="max-w-6xl xl:mx-auto sm:my-4">
            <Navigation/>
            <div class="max-w-3xl mx-auto">{children()}</div>
        </main>
    }
}

#[component]
fn Navigation() -> impl IntoView {
    view! {
        <div class="flex items-center justify-between px-1 sm:px-4">
            <nav class="flex gap-2">
                <A href="/" active_class="animate-pulse">
                    "Home"
                </A>
                <A href="/work" active_class="animate-pulse">
                    "Work"
                </A>
                <A href="/blog" active_class="animate-pulse">
                    "Blog"
                </A>
                <A href="/guestbook" active_class="animate-pulse">
                    "Guestbook"
                </A>
            </nav>
            <div class="flex gap-2">
                <a href="https://www.github.com/stefanterdell" target="_blank">
                    <Icon width="24" height="24" icon=AiGithubFilled/>
                </a>
                <a href="https://www.linkedin.com/in/stefan-terdell-58530739/" target="_blank">
                    <Icon width="24" height="24" icon=AiLinkedinFilled/>
                </a>
            </div>
        </div>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <h1>"Hi! I'm Stefan 👋"</h1>
        <p>

            "Thanks for checking out my homepage! I'm Stefan Terdell, a software developer currently working at "
            <a href="https://cleura.com" target="_blank">
                "Cleura"
            </a>
            ". In my spare time I like to be out in nature, play music (I play guitar, drums and occassionally Nyckelharpa) and work on personal side projects. I've written some open source stuff as well, like "
            <a href="https://www.npmjs.com/package/zod-to-json-schema" target="_blank">
                "zod-to-json-schema"
            </a> ". It's all on my " <a href="https://github.com/StefanTerdell" target="_blank">
                "GitHub"
            </a> "."
        </p>
        <p>
            "For some reason I always end up writing stuff related to types, schemas and DX. I started out writing C# in my spare time while playing with Unity, and later got a job writing .NET apps for Akind Group (formerly Academic Work). As a beginner I really appreciated the auto-completion and friendly compiler messages. To recreate the feeling of rapid iteration in a Unity project for mobile phones, I learned about reflection and runtime type inspection in order to build a UI overlay to tweak all of my in-game parameters. This was a good stepping stone into TypeScript later on, where the abstraction layers became even more obvious."
        </p>
        <p>
            "Lately I've been growing more and more weary of TypeScript and it's flaky ecosystem, and have been getting more and more into Rust. In fact, this website is written entirely without JavaScript (well, besides some Tailwind shenanigans), using Leptos in SSR mode. My editor of choice is Helix."
        </p>
        <p>
            "Feel free to checkout my work history, read some blog posts, and don't forget to leave a note in the guestbook ☺"
        </p>
    }
}

#[component]
fn Work() -> impl IntoView {
    view! { <div>"List of workplaces"</div> }
}

#[component]
fn Blog() -> impl IntoView {
    let articles = ARTICLES
        .iter()
        .map(|a| {
            view! {
                <h2 class="text-red-500">{a.name}</h2>
                <p>{a.content}</p>
            }
        })
        .collect_view();

    view! { <div>{articles}</div> }
}
