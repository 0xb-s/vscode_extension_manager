
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExtensionError {
    #[error("CLI command failed with status: {0}")]
    CommandFailed(String),
    #[error("Failed to parse output: {0}")]
    ParseError(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}
