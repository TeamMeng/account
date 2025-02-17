use crate::{fail, fail_null};
use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;
use tracing::info;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Argon2 password hash error: {0}")]
    Argon2Error(#[from] argon2::password_hash::Error),

    #[error("Jwt error: {0}")]
    JwtError(#[from] jwt_simple::Error),

    #[error("{0}")]
    LoginError(String),

    #[error("Phone already exists: {0}")]
    PhoneAlreadyExists(String),

    #[error("Io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("File error: {0}")]
    SerdeYamlError(#[from] serde_yaml::Error),

    #[error("Sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),
}

pub struct ErrorWarp(pub ValidationErrors);

impl IntoResponse for ErrorWarp {
    fn into_response(self) -> axum::response::Response {
        let map = self.0.field_errors();
        let mut code = 0;
        if let Some(vec) = map.values().next() {
            if let Some(v) = vec.first() {
                match v.code.parse::<usize>() {
                    Ok(c) => {
                        code = c;
                        info!("Error code: {}", code)
                    }
                    Err(_) => code = 0,
                }
            }
        }
        fail_null(code)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let code = match &self {
            Self::Argon2Error(_)
            | Self::IoError(_)
            | Self::SerdeYamlError(_)
            | Self::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::PhoneAlreadyExists(_) | Self::LoginError(_) => StatusCode::BAD_REQUEST,
            Self::JwtError(_) => StatusCode::UNAUTHORIZED,
        };

        fail(code.as_u16() as usize, self.to_string())
    }
}
