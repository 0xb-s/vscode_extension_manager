use crate::app::config::AppConfig;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub struct ConfigLoader {
    config_path: String,
}

impl ConfigLoader {
    pub fn new() -> Self {
        Self {
            config_path: "config.json".to_string(),
        }
    }

    pub fn load_config(&self) -> Result<AppConfig> {
        if !Path::new(&self.config_path).exists() {
            // Create default config if not exists
            let default_config = AppConfig::default();
            let config_json = serde_json::to_string_pretty(&default_config)?;
            fs::write(&self.config_path, config_json)?;
            return Ok(default_config);
        }

        let config_data =
            fs::read_to_string(&self.config_path).context("Failed to read config file")?;
        let config: AppConfig =
            serde_json::from_str(&config_data).context("Failed to parse config file")?;
        Ok(config)
    }
}
