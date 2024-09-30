use crate::network::NetworkError;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{Method, RequestBuilder as ReqwestRequestBuilder, Url};
use std::collections::HashMap;

pub struct RequestBuilder {
    url: String,
    method: String,
    headers: HashMap<String, String>,
    query_params: HashMap<String, String>,
    body: Option<String>,
}

impl RequestBuilder {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            method: "GET".to_string(),
            headers: HashMap::new(),
            query_params: HashMap::new(),
            body: None,
        }
    }

    pub fn method(mut self, method: &str) -> Self {
        self.method = method.to_uppercase();
        self
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn query(mut self, params: &[(&str, &str)]) -> Self {
        for (k, v) in params {
            self.query_params.insert(k.to_string(), v.to_string());
        }
        self
    }

    pub fn body(mut self, body: String) -> Self {
        self.body = Some(body);
        self
    }

    pub fn build(self) -> Result<reqwest::Request, NetworkError> {
        let method = Method::from_bytes(self.method.as_bytes()).map_err(|e| {
            NetworkError::InvalidMethod {
                method: self.method.clone(),
                source: e.to_string().into(),
            }
        })?;

        let mut url = Url::parse(&self.url).map_err(|e| NetworkError::InvalidUrl {
            url: self.url.clone(),
            source: e.to_string().into(),
        })?;

        if !self.query_params.is_empty() {
            url.query_pairs_mut().extend_pairs(&self.query_params);
        }

        let mut req_builder = reqwest::Client::new().request(method, url);

        for (key, value) in self.headers {
            let header_name = HeaderName::from_bytes(key.as_bytes()).map_err(|e| {
                NetworkError::InvalidHeader {
                    key: key.clone(),
                    value: value.clone(),
                    source: e.to_string().into(),
                }
            })?;
            let header_value =
                HeaderValue::from_str(&value).map_err(|e| NetworkError::InvalidHeader {
                    key: key.clone(),
                    value: value.clone(),
                    source: e.to_string().into(),
                })?;
            req_builder = req_builder.header(header_name, header_value);
        }

        if let Some(body_content) = self.body {
            req_builder = req_builder.body(body_content);
        }

        req_builder
            .build()
            .map_err(|e| NetworkError::RequestBuildError(e.to_string()))
    }
}
