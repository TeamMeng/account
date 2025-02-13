use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("File error: {0}")]
    SerdeYamlError(#[from] serde_yaml::Error),
}
