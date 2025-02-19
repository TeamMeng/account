mod feed;
mod follower;
mod like;
mod post;
mod user;

use crate::{time, AppState};
use anyhow::Result;
use axum::{middleware::from_fn, Router};
use feed::feed_router;
use follower::follower_router;
use like::like_router;
use post::post_router;
use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};
use user::user_router;

const ADDR: &str = "0.0.0.0:";

pub async fn start_route(state: AppState) -> Result<()> {
    let layer = Layer::new().pretty().with_filter(LevelFilter::INFO);

    tracing_subscriber::registry().with(layer).init();

    let user_routers = user_router(state.clone());
    let follower_routers = follower_router(state.clone());
    let post_routers = post_router(state.clone());
    let feed_routers = feed_router(state.clone());
    let like_routers = like_router(state.clone());

    let app = Router::new()
        .nest("/follower", follower_routers)
        .nest("/user", user_routers)
        .nest("/post", post_routers)
        .nest("/feed", feed_routers)
        .nest("/like", like_routers)
        .layer(from_fn(time));

    let addr = format!("{}{}", ADDR, state.config.server.port);
    info!("Server listening on {}", addr);

    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app).await?;

    Ok(())
}
