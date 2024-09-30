use super::{errors::ExtensionError, traits::CLICommands};
use crate::manager::models::Extension;
use anyhow::{Context, Result};

use std::process::Command;

pub struct VSCodeCommands;

impl VSCodeCommands {
    pub fn new() -> Self {
        Self
    }
}

impl CLICommands for VSCodeCommands {
    fn list_extensions(&self) -> Result<Vec<Extension>, ExtensionError> {
        let output = Command::new("code")
            .arg("--list-extensions")
            .arg("--show-versions")
            .output()
            .context("Failed to execute code --list-extensions")
            .unwrap();

        if !output.status.success() {
            return Err(ExtensionError::CommandFailed(format!(
                "code --list-extensions failed with status: {}",
                output.status
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let extensions: Vec<Extension> = stdout
            .lines()
            .map(|line| {
                let parts: Vec<&str> = line.split('@').collect();
                Extension {
                    id: parts[0].to_string(),
                    version: parts.get(1).unwrap_or(&"").to_string(),
                    installed: true,
                }
            })
            .collect();

        Ok(extensions)
    }

    fn install_extension(&self, id: &str) -> Result<(), ExtensionError> {
        let status = Command::new("code")
            .arg("--install-extension")
            .arg(id)
            .status()
            .context("Failed to execute code --install-extension")
            .unwrap();

        if !status.success() {
            return Err(ExtensionError::CommandFailed(format!(
                "code --install-extension failed with status: {}",
                status
            )));
        }

        Ok(())
    }

    fn uninstall_extension(&self, id: &str) -> Result<(), ExtensionError> {
        let status = Command::new("code")
            .arg("--uninstall-extension")
            .arg(id)
            .status()
            .context("Failed to execute code --uninstall-extension")
            .unwrap();

        if !status.success() {
            return Err(ExtensionError::CommandFailed(format!(
                "code --uninstall-extension failed with status: {}",
                status
            )));
        }

        Ok(())
    }
    fn unused_trait_method(&self) {
        todo!()
    }
}
