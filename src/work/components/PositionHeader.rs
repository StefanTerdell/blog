use super::super::models::Direction;
use leptos::*;

#[component]
pub fn PositionHeader(
    dir: Direction,
    title: String,
    company: String,
    company_url: Option<String>,
) -> impl IntoView {
    use crate::components::links::Fa;

    let company_el = match company_url {
        Some(href) => view! { <Fa href=href>{company}</Fa> }.into_view(),
        None => company.into_view(),
    };

    view! {
        <div class="font-serif text-2xl font-black md:whitespace-nowrap">
            {match dir {
                Direction::Left => {
                    view! { <>{title} ", " {company_el}</> }
                }
                Direction::Right => {
                    view! { <>{company_el} ", " {title}</> }
                }
            }}

        </div>
    }
}
