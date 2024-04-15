use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    use crate::{
        components::github::Provider as AuthProvider,
        guestbook::Guestbook,
        pages::{
            blog::{BlogPost, BlogPosts, EditBlogPost},
            home::Home,
            work::Work,
        },
        utils::error_template::{AppError, ErrorTemplate},
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
        <Stylesheet id="leptos" href="/style.css"/>
        <Router fallback=error_template>
            <AuthProvider>
                <div class="max-w-3xl mx-auto">
                    <Navigation/>
                    <main class="sm:my-4 pt-8">
                        <Routes>
                            <Route path="" view=Home/>
                            <Route path="work" view=Work/>
                            <Route ssr=SsrMode::Async path="blog" view=BlogPosts/>
                            <Route ssr=SsrMode::Async path="blog/:slug" view=BlogPost/>
                            <Route ssr=SsrMode::Async path="blog/:slug/edit" view=EditBlogPost/>
                            <Route ssr=SsrMode::Async path="guestbook" view=Guestbook/>
                        </Routes>
                    </main>
                </div>
            </AuthProvider>
        </Router>
    }
}

#[component]
fn Navigation() -> impl IntoView {
    use icondata::{AiGithubFilled, AiLinkedinFilled};
    use leptos_icons::*;
    use leptos_router::*;

    view! {
        <div class="w-full md:my-4 flex items-center justify-between">
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
                <a
                    class="tooltip tooltip-bottom"
                    data-tip="My GitHub profile"
                    href="https://www.github.com/stefanterdell"
                    target="_blank"
                >
                    <Icon width="24" height="24" icon=AiGithubFilled/>
                </a>
                <a
                    class="tooltip tooltip-bottom"
                    data-tip="My LinkedIn profile"
                    href="https://www.linkedin.com/in/stefan-terdell-58530739/"
                    target="_blank"
                >
                    <Icon width="24" height="24" icon=AiLinkedinFilled/>
                </a>
            </div>
        </div>
    }
}
