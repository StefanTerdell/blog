use leptos::*;

/// A Fancy a-tag wrapper
#[component]
pub fn Fa(#[prop(into)] href: String, children: Children) -> impl IntoView {
    view! {
        <a
            href=href.clone()
            target="_blank"
            class="tooltip tooltip-bottom bg-gradient-to-r from-primary to-secondary bg-clip-text text-transparent"
            data-tip=href
        >
            {children()}
        </a>
    }
}

/// A Fancy leptos_router::A-tag wrapper
#[component]
pub fn FA(#[prop(into)] href: String, children: Children) -> impl IntoView {
    use leptos_router::A;
    view! {
        <A
            href=href
            class="tooltip-bottom bg-gradient-to-r from-primary to-secondary bg-clip-text text-transparent"
        >
            {children()}
        </A>
    }
}
