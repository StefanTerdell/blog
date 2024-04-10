use leptos::*;

#[derive(Clone)]
struct PositionInfo {
    start_month_and_year: String,
    title: String,
    company: String,
    company_url: Option<String>,
    description: String,
}

impl PositionInfo {
    fn new(
        title: &'static str,
        company: &'static str,
        company_url: Option<&'static str>,
        start_month_and_year: &'static str,
        description: &'static str,
    ) -> Self {
        Self {
            title: title.into(),
            company: company.into(),
            company_url: company_url.map(|url| url.into()),
            start_month_and_year: start_month_and_year.into(),
            description: description.into(),
        }
    }
}

#[component]
pub fn Work() -> impl IntoView {
    let positions = vec![
        PositionInfo::new(
            "Senior Developer",
            "Cleura",
            Some("https://cleura.com/"),
            "September 2023 - Present",
            "In my current role at Cleura, I help develop our user portal, spending most of my time in our React frontend, dabbling in PHP for the first time as well. Actually finding PHP surprisingly pleasant to work with! We've got a really interesting OpenStack architecture under the hood that's been really exciting to get to know more about as well. We're all on the cloud now after all, so its feels good to finally get to know what makes it tick. Also getting to work with Gitlab CI and Terraform, as well as flexing the old OAuth knowledge while deploying Keycloak.",
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

enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn timeline_class(&self) -> &str {
        match self {
            Direction::Left => "timeline-start md:text-end",
            Direction::Right => "timeline-end md:text-start",
        }
    }

    pub fn from_index(index: usize) -> Self {
        if index % 2 == 0 {
            Self::Left
        } else {
            Self::Right
        }
    }
}

#[component]
fn PositionHeader(
    dir: Direction,
    title: String,
    company: String,
    company_url: Option<String>,
) -> impl IntoView {
    use crate::components::Fa;

    let company_el = match company_url {
        Some(href) => view! { <Fa href=href>{company}</Fa> }.into_view(),
        None => company.into_view(),
    };

    view! {
        <div class="font-serif text-2xl font-black whitespace-nowrap">
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

#[component]
fn Position(pos: PositionInfo, dir: Direction) -> impl IntoView {
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
