use super::super::models::{Direction, PositionInfo};
use super::PositionHeader::PositionHeader;

use leptos::*;
#[component]
pub fn Position(pos: PositionInfo, dir: Direction) -> impl IntoView {
    use icondata::AiRocketOutlined;
    use leptos_icons::Icon;

    view! {
        <li>
            <div class="timeline-middle">
                <Icon width="24" height="24" icon=AiRocketOutlined/>
            </div>
            <div class=format!("{} mb-10", dir.timeline_class())>
                <PositionHeader
                    dir=dir
                    title=pos.title
                    company=pos.company
                    company_url=pos.company_url
                />
                <time class="italic font-mono">{pos.start_month_and_year}</time>
                <p>{pos.description}</p>
            </div>
            <hr/>
        </li>
    }
}
