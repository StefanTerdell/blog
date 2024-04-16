use icondata::AiGithubOutlined;
use leptos::*;
use leptos_icons::*;

#[component]
pub fn GithubIconButton(
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
        <button class=class class:disabled=loading type="submit" disabled=loading>
            <div class="flex gap-2 items-center">
                <span>{text}</span>
                <Show when=move || !small>
                    <Icon width="24" height="24" icon=AiGithubOutlined/>
                </Show>
            </div>
        </button>
    }
}
