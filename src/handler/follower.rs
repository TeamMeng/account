use crate::{success, success_null, AppState, CreateFollower, ErrorWarp, User};
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    Extension, Json,
};
use validator::Validate;

pub async fn followee_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(input): Json<CreateFollower>,
) -> Response {
    if let Err(err) = input.validate() {
        return ErrorWarp(err).into_response();
    }

    match state.create_follower(input, user).await {
        Ok(_) => success_null(),
        Err(e) => e.into_response(),
    }
}

pub async fn get_all_follower_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Response {
    match state.get_all_follower(user).await {
        Ok(vec) => success(vec),
        Err(e) => e.into_response(),
    }
}

pub async fn get_all_followee_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Response {
    match state.get_all_followee(user).await {
        Ok(vec) => success(vec),
        Err(e) => e.into_response(),
    }
}

pub async fn delete_follower_handler(
    Path(id): Path<i64>,
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Response {
    match state.delete_followee(user, id).await {
        Ok(_) => success_null(),
        Err(e) => e.into_response(),
    }
}
