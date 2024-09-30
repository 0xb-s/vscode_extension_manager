use crate::network::{NetworkError, RequestBuilder};
use reqwest::Client;
use reqwest::Request;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use std::time::Duration;
#[derive(Clone)]
pub struct ApiClient {
    client: Client,
    base_url: String,
    timeout: Duration,
}

impl ApiClient {
    pub fn new(base_url: &str, timeout_secs: u64) -> Result<Self, NetworkError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .map_err(NetworkError::ReqwestError)?;

        Ok(Self {
            client,
            base_url: base_url.to_string(),
            timeout: Duration::from_secs(timeout_secs),
        })
    }

    pub fn build_request(&self, builder: RequestBuilder) -> Result<Request, NetworkError> {
        builder.build()
    }

    pub async fn send_request<T>(&self, request: Request) -> Result<T, NetworkError>
    where
        T: DeserializeOwned,
    {
        let response = self
            .client
            .execute(request)
            .await
            .map_err(NetworkError::ReqwestError)?;

        if !response.status().is_success() {
            return Err(NetworkError::RequestBuildError(format!(
                "Request failed with status: {}",
                response.status()
            )));
        }

        let parsed = response
            .json::<T>()
            .await
            .map_err(NetworkError::ReqwestError)?;

        Ok(parsed)
    }

    pub async fn fetch_data<T>(&self, endpoint: &str) -> Result<T, NetworkError>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}/{}", self.base_url, endpoint);
        let builder = RequestBuilder::new(&url).method("GET");
        let request = self.build_request(builder)?;
        self.send_request(request).await
    }

    pub async fn post_data<T, U>(&self, endpoint: &str, data: &T) -> Result<U, NetworkError>
    where
        T: Serialize,
        U: DeserializeOwned,
    {
        let url = format!("{}/{}", self.base_url, endpoint);
        let body = serde_json::to_string(data).map_err(|e| {
            NetworkError::RequestBuildError(format!(
                "Failed to serialize request body: {}",
                e.to_string()
            ))
        })?;
        let builder = RequestBuilder::new(&url)
            .method("POST")
            .header("Content-Type", "application/json")
            .body(body);
        let request = self.build_request(builder)?;
        self.send_request(request).await
    }

    pub async fn get_latest_version(
        &self,
        extension_id: &str,
    ) -> Result<Option<String>, NetworkError> {
        let endpoint = format!("extensions/{}/latest", extension_id);

        let response: LatestVersionResponse = self.fetch_data(&endpoint).await?;

        Ok(Some(response.version))
    }
}
#[derive(Debug, Deserialize)]
struct LatestVersionResponse {
    version: String,
}
