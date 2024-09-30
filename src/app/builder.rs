use super::config::AppConfig;
use super::App;
use crate::network::api_client::ApiClient;
use anyhow::Result;

pub struct AppBuilder {
    title: String,
    config: Option<AppConfig>,
    api_client: Option<ApiClient>,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self {
            title: "VSCode Extension Manager".to_string(),
            config: None,
            api_client: None,
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn config(mut self, config: AppConfig) -> Self {
        self.config = Some(config);
        self
    }

    pub fn api_client(mut self, api_client: ApiClient) -> Self {
        self.api_client = Some(api_client);
        self
    }

    pub fn build(self) -> Result<App> {
        let config = self.config.unwrap_or_else(|| AppConfig::default());
        let api_client = self.api_client.unwrap();

        Ok(App {
            state: super::state::AppState::new(),
            title: self.title,
            config,
            api_client,
        })
    }
}
