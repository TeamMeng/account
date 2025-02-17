use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::prelude::FromRow;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateFollower {
    #[validate(range(min = 1, code = "15002"))]
    pub followee: i32,
}

#[derive(Debug, Deserialize, FromRow)]
pub struct Follower {
    pub id: i64,
    pub follower_id: i64,
    pub followee_id: i64,
    pub created_at: DateTime<Utc>,
    pub is_deleted: bool,
}

impl CreateFollower {
    pub fn new(followee: i32) -> Self {
        Self { followee }
    }
}
