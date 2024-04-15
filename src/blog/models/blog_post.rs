use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlogPost {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub views: i32,
    pub md_content: String,
    pub html_content: String,
    pub published: bool,
    pub published_time: DateTime<Utc>,
    pub edited_time: Option<DateTime<Utc>>,
}
