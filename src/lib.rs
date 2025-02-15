mod config;
mod error;
mod handler;
mod model;
mod router;
mod service;
mod utils;

use anyhow::Result;
use router::start_route;
use sqlx::PgPool;
use sqlx_db_tester::TestPg;
use std::{ops::Deref, path::Path, sync::Arc};

pub use config::{code_init, config_init, AppConfig, AuthConfig, ServerConfig, STATUS_CODE};
pub use error::{AppError, ErrorWarp};
pub use handler::create_user_handler;
pub use model::{ChangeUserPassword, CreateUser, User};
pub use utils::{
    fail, fail_null, hash_password, local_timestamp, success, success_null, validate_phone,
    verify_password,
};

pub async fn run() -> Result<()> {
    let config = AppConfig::load()?;
    let state = AppState::new(config).await?;

    config_init().await;

    start_route(state).await?;
    Ok(())
}

#[derive(Clone)]
pub struct AppState {
    pub inner: Arc<AppStateInner>,
}

pub struct AppStateInner {
    pub config: AppConfig,
    pub pool: PgPool,
}

impl AppState {
    pub async fn new(config: AppConfig) -> Result<Self, AppError> {
        let pool = PgPool::connect(&config.server.db_url).await?;
        Ok(Self {
            inner: Arc::new(AppStateInner { config, pool }),
        })
    }

    pub async fn new_for_test() -> Result<(TestPg, Self), AppError> {
        let config = AppConfig::load()?;

        let post = config
            .server
            .db_url
            .rfind('/')
            .expect("Database url should invalid");

        let database_url = &config.server.db_url[..post];
        let tdb = TestPg::new(database_url.to_string(), Path::new("./migrations"));

        let pool = tdb.get_pool().await;

        Ok((
            tdb,
            Self {
                inner: Arc::new(AppStateInner { config, pool }),
            },
        ))
    }
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
