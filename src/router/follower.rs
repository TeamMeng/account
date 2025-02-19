use crate::{
    delete_follower_handler, followee_handler, get_all_followee_handler, get_all_follower_handler,
    verify_token, AppState,
};
use axum::{
    middleware::from_fn_with_state,
    routing::{delete, get, post},
    Router,
};

pub fn follower_router(state: AppState) -> Router {
    Router::new()
        .route("/create", post(followee_handler))
        .route("/geter", get(get_all_follower_handler))
        .route("/getee", get(get_all_followee_handler))
        .route("/delete/{id}", delete(delete_follower_handler))
        .layer(from_fn_with_state(state.clone(), verify_token))
        .with_state(state)
}
