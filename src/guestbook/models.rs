use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuestbookPost {
    pub id: i32,
    pub user_id: i32,
    pub user_name: String,
    pub user_url: String,
    pub content: String,
    pub published: bool,
    pub created_time: DateTime<Utc>,
}
