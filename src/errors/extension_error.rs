use thiserror::Error;
#[allow(dead_code)]
#[derive(Error, Debug)]
enum ExtensionError {
    #[error("CLI command failed: {0}")]
    CommandFailed(String),

    #[error("Extension not found: {0}")]
    NotFound(String),

    #[error("Cache error: {0}")]
    CacheError(String),

    #[error("API error: {0}")]
    ApiError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}
