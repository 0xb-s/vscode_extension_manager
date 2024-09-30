// src/network/errors.rs

use thiserror::Error;

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("Invalid HTTP method: {method}. Error: {source}")]
    InvalidMethod {
        method: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Invalid URL: {url}. Error: {source}")]
    InvalidUrl {
        url: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Invalid header {key}: {value}. Error: {source}")]
    InvalidHeader {
        key: String,
        value: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Failed to build the request: {0}")]
    RequestBuildError(String),

    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Serialization/Deserialization error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("Unknown network error: {0}")]
    Unknown(String),
}
