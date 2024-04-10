use leptos::*;

#[component]
pub fn Home() -> impl IntoView {
    use crate::components::{Fa, FA};

    view! {
        <article class="prose max-w-6xl">
            <h1 class="mb-2 text-5xl font-serif">"Hi! I'm Stefan"</h1>
            <p class="text-justify mb-2">
                "Thanks for checking out my homepage! I'm Stefan Terdell, a software developer currently working at "
                <Fa href="https://cleura.com">"Cleura"</Fa>
                ". In my spare time I like to be out in nature, play music (I play guitar, drums and occassionally Nyckelharpa) and work on personal side projects. I've written some open source stuff as well, like "
                <Fa href="https://www.npmjs.com/package/zod-to-json-schema">
                    "zod-to-json-schema"
                </Fa> ". It's all on my " <Fa href="https://github.com/StefanTerdell">"GitHub"</Fa>
                "."
            </p>
            <p class="text-justify mb-2">
                "Feel free to checkout my work history, read some blog posts, and don't forget to leave a note in the "
                <FA href="/guestbook">"guestbook"</FA> " :)"
            </p>
            <img class="mx-auto mt-8 mb-4" src="under-construction.gif" alt="Under construction"/>
            <i>"This site was made with " <Fa href="https://www.leptos.dev/">"Leptos"</Fa></i>
            <div class="animate-bounce">"🦀"</div>
        </article>
    }
}
