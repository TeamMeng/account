use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct Feed {
    pub fid: i64,
    pub pid: i64,
    pub uid: i64,
    pub created_at: DateTime<Utc>,
}
