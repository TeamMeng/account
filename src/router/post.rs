use crate::{
    create_post_handler, delete_post_handler, get_all_posts_handler, get_post_handler,
    verify_token, AppState,
};
use axum::{
    middleware::from_fn_with_state,
    routing::{delete, get, post},
    Router,
};

pub fn post_router(state: AppState) -> Router {
    Router::new()
        .route("/create", post(create_post_handler))
        .route("/delete/{pid}", delete(delete_post_handler))
        .layer(from_fn_with_state(state.clone(), verify_token))
        .route("/get/{uid}", get(get_all_posts_handler))
        .route("/get/{pid}/{uid}", get(get_post_handler))
        .with_state(state)
}
