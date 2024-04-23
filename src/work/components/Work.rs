use leptos::*;
use super::{Position::Position, super::models::{PositionInfo, Direction}};


#[component]
pub fn Work() -> impl IntoView {
let positions = vec![
    PositionInfo::new(
        "Senior Developer",
        "Cleura",
        Some("https://cleura.com/"),
        "September 2023 - Present",
        "In my current role at Cleura, I help develop our user portal, spending most of my time in our React frontend, dabbling in PHP for the first time as well. Actually finding PHP surprisingly pleasant to work with! We've got a really interesting OpenStack architecture under the hood that's been really exciting to get to know more about as well. We're all on the cloud now after all, so its feels good to finally get to know what makes it tick. Also getting to work with CI/CD and general devops, as well as flexing the old OAuth knowledge while deploying Keycloak.",
    ),
    PositionInfo::new(
        "Lead Developer",
        "Roboten",
        Some("https://www.roboten.com/"),
        "March 2022 - August 2023",
        "At Roboten, I was once again heavily involved in schema-driven UI generation, as well as workflow automation and deployment, and advanced frontend components - all part of a comprehensive low-code solution ecosystem. Learned a lot about authentication management too, especially SSO and the OIDC/OAuth2 protocols. My role extended to influencing our architectural direction through technical and market research, encompassing areas from frontend frameworks to code generation and collaborative editing systems design using CRDTs. This was really intense and a ton of fun!"
    ),
    PositionInfo::new(
        "Technical Lead", "Academic Work",
        Some("https://www.academicwork.com/"),
        "June 2019 - March 2022",
        "As tech lead, I helped spearhead the development of a new internal platform based around Domain Driven Design and event sourcing. We modeled data in a Neo4j graph database and handled event flows first through EventStore and later through an in-house event engine based on PSQL. Got to dabble with Protobufs and MongoDB as well, and delivered a few really cool DX tools, such as a graphical node editor in React and a code generation tool for TS types. I really appreciated being an involved part in such a large and technically diverse project - this was probably the most intensive learning period of my career so far.",
    ),
    PositionInfo::new(
        "Unity/C# Teacher",
        "Self Employed",
        None, 
        "August 2018 - March 2020",
        "In the magical pre-covid times, I conducted workshops focused on the fundamentals of game development, empowering fellow devs to start creating their own games using Unity and C#. Made some neat tooling along the way, some of which are still on my GitHub, although not maintained. It was definitely a fun foray into entrepreneurship, but most of all, I just love teaching people new stuff!".into(),
    ),
    PositionInfo::new (
        "Web Developer",
        "Academic Work",
        Some("https://www.academicwork.com/"),
        "January 2018 - June 2019",
        "During my time at Academic Work, I was involved in writing, deploying, and maintaining a variety of internal applications and services using C#/.Net and Vue.js. I explored and implemented Application Performance Management tooling and championed agile methodologies, working closely with product owners and the rest of the team to refine our day-to-day work.",
    ),
    PositionInfo::new(
        "Team Lead",
        "Coor",
        Some("https://www.coor.com/"),
        "February 2015 - January 2018",
        "At Coor's Service Centre & Logistics, I managed a six-person team providing first-line technical support at the headquarters of one of Sweden's leading telecom companies, involved in technical assistance and logistical operations.",
    ),
]
.into_iter()
.enumerate().map(|(index, pos)| view! { <Position pos=pos dir=Direction::from_index(index)/> })
.collect_view();

view! {
    <div>
        <ul class="timeline timeline-snap-icon max-md:timeline-compact timeline-vertical">
            {positions}
        </ul>
    </div>
}
}
