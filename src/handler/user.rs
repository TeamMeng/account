use crate::{success, AppState, CreateUser, ErrorWarp};
use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use validator::Validate;

pub async fn create_user_handler(
    State(state): State<AppState>,
    Json(input): Json<CreateUser>,
) -> Response {
    if let Err(err) = input.validate() {
        return ErrorWarp(err).into_response();
    }

    match state.create_user(input).await {
        Ok(user) => success(user.uid),
        Err(e) => e.into_response(),
    }
}
