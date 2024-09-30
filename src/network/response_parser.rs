use crate::network::NetworkError;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct ResponseParser;

impl ResponseParser {

    pub fn parse_response<T>(response_body: &str) -> Result<T, NetworkError>
    where
        T: DeserializeOwned,
    {
        serde_json::from_str::<T>(response_body).map_err(|e| {
            NetworkError::RequestBuildError(format!(
                "Failed to parse response body: {}",
                e.to_string()
            ))
        })
    }


    pub fn serialize_request<T>(data: &T) -> Result<String, NetworkError>
    where
        T: Serialize,
    {
        serde_json::to_string(data).map_err(|e| {
            NetworkError::RequestBuildError(format!(
                "Failed to serialize request body: {}",
                e.to_string()
            ))
        })
    }
}
