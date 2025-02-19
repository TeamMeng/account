use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use validator::Validate;

#[derive(Debug, Serialize, FromRow)]
pub struct Like {
    pub lid: i64,
    pub pid: i64,
    pub uid: i64,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateLike {
    #[validate(range(min = 1, code = "17001"))]
    pub pid: i64,
}
