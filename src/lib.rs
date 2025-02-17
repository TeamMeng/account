mod config;
mod error;
mod handler;
mod middleware;
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
pub use handler::{create_user_handler, signin_handler};
pub use middleware::{time, verify_token};
pub use model::{ChangeUserPassword, CreateUser, RespToken, SigninUser, User};
pub use utils::{
    fail, fail_null, hash_password, local_timestamp, success, success_null, validate_phone,
    verify_password, DecodingKey, EncodingKey,
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
    pub ek: EncodingKey,
    pub dk: DecodingKey,
}

impl AppState {
    pub async fn new(config: AppConfig) -> Result<Self, AppError> {
        let pool = PgPool::connect(&config.server.db_url).await?;
        let encoding_pem = include_str!("../fixtures/encoding.pem");
        let decoding_pem = include_str!("../fixtures/decoding.pem");
        let ek = EncodingKey::load(encoding_pem)?;
        let dk = DecodingKey::load(decoding_pem)?;
        Ok(Self {
            inner: Arc::new(AppStateInner {
                config,
                pool,
                ek,
                dk,
            }),
        })
    }

    pub async fn new_for_test() -> Result<(TestPg, Self), AppError> {
        let config = AppConfig::load()?;
        let encoding_pem = include_str!("../fixtures/encoding.pem");
        let decoding_pem = include_str!("../fixtures/decoding.pem");
        let ek = EncodingKey::load(encoding_pem)?;
        let dk = DecodingKey::load(decoding_pem)?;

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
                inner: Arc::new(AppStateInner {
                    config,
                    pool,
                    ek,
                    dk,
                }),
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
