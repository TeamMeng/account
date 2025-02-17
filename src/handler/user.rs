use crate::{success, AppState, CreateUser, ErrorWarp, RespToken, SigninUser};
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

pub async fn signin_handler(
    State(state): State<AppState>,
    Json(input): Json<SigninUser>,
) -> Response {
    if let Err(err) = input.validate() {
        return ErrorWarp(err).into_response();
    }

    match state.signin(input).await {
        Ok(user) => match state.ek.sign(user) {
            Ok(token) => {
                let ret = RespToken::new(token);
                success(ret)
            }
            Err(e) => e.into_response(),
        },
        Err(e) => e.into_response(),
    }
}

pub async fn get_all_users_handler(State(state): State<AppState>) -> Response {
    match state.get_all_users().await {
        Ok(user) => success(user),
        Err(e) => e.into_response(),
    }
}
