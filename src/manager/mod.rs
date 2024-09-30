
pub mod cache;
pub mod commands;
pub mod errors;
pub mod models;
pub mod traits;
pub mod updater;
use anyhow::{Context, Result};

use cache::CacheManager;
use commands::VSCodeCommands;
use errors::ExtensionError;
use models::Extension;

use serde_json::Value;
use std::collections::HashMap;
use traits::CLICommands;
use updater::ExtensionUpdater;

use crate::network::ApiClient;

pub struct ExtensionManager {
    pub extensions: Vec<Extension>,
    cli: VSCodeCommands,
    extension_map: HashMap<String, Extension>,
    cache_manager: CacheManager,
    updater: ExtensionUpdater,
    api_client: ApiClient,
}

impl ExtensionManager {
    pub fn new(api_client: ApiClient) -> Self {
        let cli = VSCodeCommands::new();
        let cache_manager = CacheManager::new();
        let updater = ExtensionUpdater::new();
        let extensions = cli.list_extensions().unwrap_or_default();
        let extension_map = extensions
            .iter()
            .map(|ext| (ext.id.clone(), ext.clone()))
            .collect();

        Self {
            extensions,
            cli,
            extension_map,
            cache_manager,
            updater,
            api_client,
        }
    }

    pub fn get_extension(&self, index: usize) -> Option<&Extension> {
        self.extensions.get(index)
    }

    pub fn get_extension_id(&self, index: usize) -> Option<&String> {
        self.extensions.get(index).map(|ext| &ext.id)
    }

    pub async fn toggle_extension(&mut self, extension_id: &str) -> Result<(), ExtensionError> {
        if let Some(ext) = self.extension_map.get(extension_id) {
            if ext.installed {
                self.uninstall_extension(extension_id).await?;
            } else {
                self.install_extension(extension_id).await?;
            }
            self.refresh_extensions();
            Ok(())
        } else {
            Err(ExtensionError::Unknown(extension_id.to_string()))
        }
    }

    pub async fn install_extension(&mut self, id: &str) -> Result<(), ExtensionError> {
        self.cli.install_extension(id).map_err(|e| {
            log::error!("Failed to install {}: {:?}", id, e);
            ExtensionError::CommandFailed(e.to_string())
        })?;
        Ok(())
    }

    pub async fn uninstall_extension(&mut self, id: &str) -> Result<(), ExtensionError> {
        self.cli.uninstall_extension(id).map_err(|e| {
            log::error!("Failed to uninstall {}: {:?}", id, e);
            ExtensionError::CommandFailed(e.to_string())
        })?;
        Ok(())
    }

    pub fn refresh_extensions(&mut self) {
        match self.cli.list_extensions() {
            Ok(exts) => {
                self.extensions = exts;
                self.extension_map = self
                    .extensions
                    .iter()
                    .map(|ext| (ext.id.clone(), ext.clone()))
                    .collect();
            }
            Err(e) => log::error!("Failed to refresh extensions: {:?}", e),
        }
    }

    pub async fn update_extensions(&mut self) -> Result<(), ExtensionError> {
        self.updater
            .update_all_extensions(&mut self.cli, &self.api_client)
            .await
    }

    pub fn clear_cache(&mut self) -> Result<(), ExtensionError> {
        self.cache_manager.clear_cache().map_err(|e| {
            log::error!("Failed to clear cache: {:?}", e);
            ExtensionError::ParseError(e.to_string())
        })
    }

  
}
