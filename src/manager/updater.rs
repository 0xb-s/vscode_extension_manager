use super::commands::VSCodeCommands;
use super::errors::ExtensionError;
use super::traits::CLICommands;
use crate::network::api_client::ApiClient;
use anyhow::Result;
pub struct ExtensionUpdater;

impl ExtensionUpdater {
    pub fn new() -> Self {
        Self
    }

    pub async fn update_all_extensions(
        &self,
        cli: &VSCodeCommands,
        api_client: &ApiClient,
    ) -> Result<(), ExtensionError> {
        let extensions = cli.list_extensions()?;
        for ext in extensions {
            if let Some(latest_version) = api_client.get_latest_version(&ext.id).await.unwrap() {
                if latest_version > ext.version {
                    cli.install_extension(&ext.id)?;
                    log::info!("Updated extension: {}", ext.id);
                }
            }
        }
        Ok(())
    }
}
