use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateBlogPostPayload {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub md_content: String,
    pub published: Option<String>,
}
