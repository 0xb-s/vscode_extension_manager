use super::errors::ExtensionError;
use crate::manager::models::Extension;
use anyhow::Result;

pub trait CLICommands {
    fn list_extensions(&self) -> Result<Vec<Extension>, ExtensionError>;
    fn install_extension(&self, id: &str) -> Result<(), ExtensionError>;
    fn uninstall_extension(&self, id: &str) -> Result<(), ExtensionError>;


    fn unused_trait_method(&self);
}
