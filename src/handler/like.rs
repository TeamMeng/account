use crate::{success, success_null, AppState, CreateLike, ErrorWarp, User};
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    Extension, Json,
};
use validator::Validate;

pub async fn create_like_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(input): Json<CreateLike>,
) -> Response {
    if let Err(e) = input.validate() {
        return ErrorWarp(e).into_response();
    }

    match state.create_like(user.uid, input.pid).await {
        Ok(_) => success_null(),
        Err(e) => e.into_response(),
    }
}

pub async fn delete_like_handler(
    Path(pid): Path<i64>,
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Response {
    match state.delete_like(user.uid, pid).await {
        Ok(_) => success_null(),
        Err(e) => e.into_response(),
    }
}

pub async fn get_likes_num_handler(
    Path(pid): Path<i64>,
    State(state): State<AppState>,
) -> Response {
    match state.get_likes_num(pid).await {
        Ok(ret) => success(ret),
        Err(e) => e.into_response(),
    }
}
