use crate::AppError;
use serde::Deserialize;
use std::fs::File;

#[derive(Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub auth: AuthConfig,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub db_url: String,
}

#[derive(Deserialize)]
pub struct AuthConfig {
    pub ek: String,
    pub dk: String,
}

impl AppConfig {
    pub fn load() -> Result<Self, AppError> {
        let rdr = File::open("backend.yaml")?;
        let ret = serde_yaml::from_reader(rdr)?;
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn app_config_load_should_work() -> Result<()> {
        let config = AppConfig::load()?;

        assert_eq!(config.server.port, 6688);
        assert_eq!(
            config.server.db_url,
            "postgres://postgres:postgres@localhost:5432/account"
        );
        assert_eq!(config.auth.ek, "-----BEGIN PRIVATE KEY-----\nMC4CAQAwBQYDK2VwBCIEIO86NLYAOor1kUohceuaT9susMROxY973ceRUg+LQx97\n-----END PRIVATE KEY-----\n");
        assert_eq!(config.auth.dk, "-----BEGIN PUBLIC KEY-----\nMCowBQYDK2VwAyEAlCHtaGQUJ64HH7fP2rxuqkhoOl6mEYbNJbPuvAdao6I=\n-----END PUBLIC KEY-----\n");
        Ok(())
    }
}
