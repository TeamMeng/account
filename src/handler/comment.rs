use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    Extension, Json,
};
use validator::Validate;

use crate::{success, success_null, AppState, CreateComment, ErrorWarp, User};

pub async fn create_comment_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(input): Json<CreateComment>,
) -> Response {
    if let Err(e) = input.validate() {
        return ErrorWarp(e).into_response();
    }

    match state
        .create_comment(input.pid, user.uid, &input.content)
        .await
    {
        Ok(comment) => success(comment),
        Err(e) => e.into_response(),
    }
}

pub async fn delete_comment_handler(
    Path(pid): Path<i64>,
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Response {
    match state.delete_comment(pid, user.uid).await {
        Ok(_) => success_null(),
        Err(e) => e.into_response(),
    }
}

pub async fn get_all_comments_handler(
    Path(pid): Path<i64>,
    State(state): State<AppState>,
) -> Response {
    match state.get_all_comments(pid).await {
        Ok(comments) => success(comments),
        Err(e) => e.into_response(),
    }
}
