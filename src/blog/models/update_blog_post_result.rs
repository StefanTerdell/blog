use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateBlogPostResult {
    pub saved_time: DateTime<Utc>,
    pub slug: String,
}
