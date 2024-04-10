use leptos::*;

/// A Fancy a-tag wrapper
#[component]
pub fn Fa(#[prop(into)] href: String, children: Children) -> impl IntoView {
    view! {
        <a
            href=href.clone()
            target="_blank"
            class="tooltip tooltip-bottom fancy-link"
            data-tip=href
        >
            {children()}
        </a>
    }
}

/// A Fancy leptos_router::A-tag wrapper
#[component]
pub fn FA(
    #[prop(optional, into)] class: String,
    #[prop(into)] href: String,
    children: Children,
) -> impl IntoView {
    use leptos_router::A;
    view! {
        <span class="tooltip tooltip-bottom" data-tip=href.clone()>
            <A href=href class=format!("{class} fancy-link")>
                {children()}
            </A>
        </span>
    }
}
