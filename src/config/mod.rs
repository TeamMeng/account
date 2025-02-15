mod app_config;
mod status_code;

pub use app_config::{AppConfig, AuthConfig, ServerConfig};
pub use status_code::{code_init, STATUS_CODE};

pub async fn config_init() {
    code_init().await;
}
