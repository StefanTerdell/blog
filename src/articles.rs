pub struct Article<'a> {
    pub name: &'a str,
    pub content: &'a str,
}

pub const ARTICLES: [Article; 2] = [
    Article {
        name: "My Article",
        content: include_str!("../articles/my_article.md"),
    },
    Article {
        name: "My Other Article",
        content: include_str!("../articles/my_other_article.md"),
    },
];
