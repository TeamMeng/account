use crate::{success, AppState, ErrorWarp, ReqFeed, User};
use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Extension, Json,
};
use validator::Validate;

pub async fn feeds_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(feed): Json<ReqFeed>,
) -> Response {
    if let Err(e) = feed.validate() {
        return ErrorWarp(e).into_response();
    }

    match state.get_feeds(user.uid, feed.pid, feed.size).await {
        Ok(post) => success(post),
        Err(e) => e.into_response(),
    }
}
