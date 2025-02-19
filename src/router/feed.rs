use crate::{feeds_handler, verify_token, AppState};
use axum::{middleware::from_fn_with_state, routing::get, Router};

pub fn feed_router(state: AppState) -> Router {
    Router::new()
        .route("/get", get(feeds_handler))
        .layer(from_fn_with_state(state.clone(), verify_token))
        .with_state(state)
}
