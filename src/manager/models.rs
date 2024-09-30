use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extension {
    pub id: String,
    pub version: String,
    pub installed: bool,

}

impl Extension {
    pub fn new(id: &str, version: &str, installed: bool) -> Self {
        Self {
            id: id.to_string(),
            version: version.to_string(),
            installed,
        }
    }


    pub fn builder(id: &str, version: &str) -> ExtensionBuilder {
        ExtensionBuilder::new(id, version)
    }
}

pub struct ExtensionBuilder {
    id: String,
    version: String,
    installed: bool,
}

impl ExtensionBuilder {
    pub fn new(id: &str, version: &str) -> Self {
        Self {
            id: id.to_string(),
            version: version.to_string(),
            installed: false,
        }
    }

    pub fn installed(mut self, installed: bool) -> Self {
        self.installed = installed;
        self
    }

    pub fn build(self) -> Extension {
        Extension {
            id: self.id,
            version: self.version,
            installed: self.installed,
        }
    }
}
