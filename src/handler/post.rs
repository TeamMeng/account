use crate::{fail_null, success, success_null, AppState, CreatePost, ErrorWarp, User};
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    Extension, Json,
};
use validator::Validate;

pub async fn create_post_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(post): Json<CreatePost>,
) -> Response {
    if let Err(e) = post.validate() {
        return ErrorWarp(e).into_response();
    }

    match state.create_post(user, &post.content).await {
        Ok(post) => success(post),
        Err(e) => e.into_response(),
    }
}

pub async fn delete_post_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Path(pid): Path<i64>,
) -> Response {
    match state.delete_post(user, pid).await {
        Ok(_) => success_null(),
        Err(e) => e.into_response(),
    }
}

pub async fn get_all_posts_handler(
    State(state): State<AppState>,
    Path(uid): Path<i64>,
) -> Response {
    match state.get_all_posts(uid).await {
        Ok(posts) => success(posts),
        Err(e) => e.into_response(),
    }
}

pub async fn get_post_handler(
    State(state): State<AppState>,
    Path((pid, uid)): Path<(i64, i64)>,
) -> Response {
    match state.find_post(pid, uid).await {
        Ok(ret) => match ret {
            Some(post) => success(post),
            None => fail_null(16002),
        },
        Err(e) => e.into_response(),
    }
}
