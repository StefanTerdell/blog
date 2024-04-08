use leptos::*;

#[component]
pub fn Fa(#[prop(into)] href: String, children: Children) -> impl IntoView {
    view! {
        <a
            href=href
            target="_blank"
            class="bg-gradient-to-r from-primary to-secondary bg-clip-text text-transparent"
        >
            {children()}
        </a>
    }
}
