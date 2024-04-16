use super::{LogInButton, LogOutButton, LoggedIn};
use leptos::*;

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
