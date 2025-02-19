use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use validator::Validate;

#[derive(Debug, Serialize, FromRow)]
pub struct Comment {
    pub cid: i64,
    pub pid: i64,
    pub uid: i64,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateComment {
    #[validate(range(min = 1, code = "17001"))]
    pub pid: i64,
    #[validate(length(min = 1, max = 140, code = "16001"))]
    pub content: String,
}
