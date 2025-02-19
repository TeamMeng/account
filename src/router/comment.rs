use crate::{
    create_comment_handler, delete_comment_handler, get_all_comments_handler, verify_token,
    AppState,
};
use axum::{
    middleware::from_fn_with_state,
    routing::{delete, get, post},
    Router,
};

pub fn comment_handler(state: AppState) -> Router {
    Router::new()
        .route("/create", post(create_comment_handler))
        .route("/delete/{pid}", delete(delete_comment_handler))
        .layer(from_fn_with_state(state.clone(), verify_token))
        .route("/get/{pid}", get(get_all_comments_handler))
        .with_state(state)
}
