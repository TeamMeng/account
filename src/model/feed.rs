use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use validator::Validate;

#[derive(Debug, Serialize, FromRow)]
pub struct Feed {
    pub fid: i64,
    pub pid: i64,
    pub uid: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ReqFeed {
    #[validate(range(min = 1, code = "17001"))]
    pub pid: i64,
    #[validate(range(min = 1, max = 50, code = "17002"))]
    pub size: usize,
}
