use crate::AppState;
use anyhow::Result;
use axum::Router;
use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};
use user::user_router;

mod user;

const ADDR: &str = "0.0.0.0:";

pub async fn start_route(state: AppState) -> Result<()> {
    let layer = Layer::new().pretty().with_filter(LevelFilter::INFO);

    tracing_subscriber::registry().with(layer).init();

    let user_routers = user_router(state.clone());
    let app = Router::new().nest("/user", user_routers);

    let addr = format!("{}{}", ADDR, state.config.server.port);
    info!("Server listening on {}", addr);

    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app).await?;

    Ok(())
}
