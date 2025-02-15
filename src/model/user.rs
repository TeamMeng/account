use crate::validate_phone;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub uid: i64,
    pub nickname: String,
    pub phone: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, FromRow, Clone, Validate)]
pub struct CreateUser {
    #[validate(custom(function = "validate_phone", code = "10001"))]
    pub phone: String,
    #[validate(length(min = 6, max = 20, code = "10002"))]
    pub password: String,
    pub nickname: String,
}

#[derive(Debug, Deserialize, FromRow, Clone)]
pub struct ChangeUserPassword {
    pub password: String,
}

impl ChangeUserPassword {
    pub fn new(password: &str) -> Self {
        Self {
            password: password.to_string(),
        }
    }
}

impl CreateUser {
    pub fn new(phone: &str, password: &str, nickname: &str) -> Self {
        Self {
            phone: phone.to_string(),
            password: password.to_string(),
            nickname: nickname.to_string(),
        }
    }
}
