use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePost {
    #[validate(length(min = 1, max = 140, code = "16001"))]
    pub content: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Post {
    pub pid: i64,
    pub uid: i64,
    pub content: String,
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CreatePost {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
        }
    }
}
