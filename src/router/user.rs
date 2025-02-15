use crate::{create_user_handler, AppState};
use axum::{routing::post, Router};

pub fn user_router(state: AppState) -> Router {
    Router::new()
        .route("/create", post(create_user_handler))
        .with_state(state)
}
