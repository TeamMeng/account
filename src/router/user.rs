use crate::{create_user_handler, signin_handler, verify_token, AppState};
use axum::{middleware::from_fn_with_state, routing::post, Router};

pub fn user_router(state: AppState) -> Router {
    Router::new()
        .layer(from_fn_with_state(state.clone(), verify_token))
        .route("/create", post(create_user_handler))
        .route("/signin", post(signin_handler))
        .with_state(state)
}
