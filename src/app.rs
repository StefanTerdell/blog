use leptos::*;

mod blog;
mod guestbook;
mod work;

#[component]
pub fn App() -> impl IntoView {
    use crate::{
        app::blog::{Post, Posts},
        app::work::Work,
        error_template::{AppError, ErrorTemplate},
        github::{Callback as AuthCallback, Provider as AuthProvider},
    };
    use leptos_meta::*;
    use leptos_router::*;

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
                <AuthProvider>
                    <Routes>
                        <Route path="" view=Home/>
                        <Route path="work" view=Work/>
                        <Route ssr=SsrMode::Async path="blog" view=Posts/>
                        <Route ssr=SsrMode::Async path="blog/:slug" view=Post/>
                        <Route ssr=SsrMode::Async path="guestbook" view=guestbook::Guestbook/>
                        <Route path="callback" view=AuthCallback/>
                    </Routes>
                </AuthProvider>
            </Layout>
        </Router>
    }
}

#[component]
fn Layout(children: Children) -> impl IntoView {
    view! {
        <Navigation/>
        <main class="max-w-6xl xl:mx-auto sm:my-4 pt-8">
            <div class="max-w-3xl mx-auto">{children()}</div>
        </main>
    }
}

#[component]
fn Navigation() -> impl IntoView {
    use icondata::{AiGithubFilled, AiLinkedinFilled};
    use leptos_icons::*;
    use leptos_router::*;

    view! {
        <div class="w-full xl:mx-auto sm:my-4 flex items-center justify-between px-1 sm:px-4 fixed top-0">
            <nav class="flex gap-2">
                <A href="/" active_class="underline">
                    "Home"
                </A>
                <A href="/work" active_class="underline">
                    "Work"
                </A>
                <A href="/blog" active_class="underline">
                    "Blog"
                </A>
                <A href="/guestbook" active_class="underline">
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
    use crate::components::Fa;

    view! {
        <h1 class="text-2xl my-8">"Hi! I'm Stefan 👋"</h1>
        <p class="text-justify mb-2">
            "Thanks for checking out my homepage! I'm Stefan Terdell, a software developer currently working at "
            <Fa href="https://cleura.com">"Cleura"</Fa>
            ". In my spare time I like to be out in nature, play music (I play guitar, drums and occassionally Nyckelharpa) and work on personal side projects. I've written some open source stuff as well, like "
            <Fa href="https://www.npmjs.com/package/zod-to-json-schema">"zod-to-json-schema"</Fa>
            ". It's all on my " <Fa href="https://github.com/StefanTerdell">"GitHub"</Fa> "."
        </p>
        <p class="text-justify mb-2">
            "Feel free to checkout my work history, read some blog posts, and don't forget to leave a note in the guestbook :)"
        </p>
        <img class="mx-auto my-4" src="under-construction.gif" alt="Under construction"/>
        <i>"This site was made with " <Fa href="https://www.leptos.dev/">"Leptos"</Fa></i>
        <div class="animate-bounce">"🦀"</div>
    }
}
