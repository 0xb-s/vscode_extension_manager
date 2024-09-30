use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub api_endpoint: String,
    pub refresh_interval: u64,
    pub theme: Theme,
    pub timeout_secs: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            api_endpoint: "".to_string(),
            refresh_interval: 300, 
            theme: Theme::Dark,
            timeout_secs: 10,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Debug, Clone)]
pub enum ViewMode {
    List,
    Grid,
}
