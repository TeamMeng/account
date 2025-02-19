use crate::{
    create_like_handler, delete_like_handler, get_likes_num_handler, verify_token, AppState,
};
use axum::{
    middleware::from_fn_with_state,
    routing::{delete, get, post},
    Router,
};

pub fn like_router(state: AppState) -> Router {
    Router::new()
        .route("/create", post(create_like_handler))
        .route("/delete/{pid}", delete(delete_like_handler))
        .layer(from_fn_with_state(state.clone(), verify_token))
        .route("/get/{pid}", get(get_likes_num_handler))
        .with_state(state)
}
