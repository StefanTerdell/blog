use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct BlogPostListItem {
    pub slug: String,
    pub title: String,
    pub views: i64,
    pub published_time: DateTime<Utc>,
}
