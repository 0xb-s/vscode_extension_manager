use super::errors::ExtensionError;
use super::models::Extension;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CacheData {
    pub extensions: Vec<Extension>,
}

pub struct CacheManager {
    cache: HashMap<String, CacheData>,
}

impl CacheManager {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn _load_cache(&self, key: &str) -> Result<CacheData, ExtensionError> {
        self.cache
            .get(key)
            .cloned()
            .ok_or_else(|| ExtensionError::ParseError(format!("No cache found for key: {}", key)))
    }

    pub fn _save_cache(&mut self, key: &str, data: CacheData) -> Result<(), ExtensionError> {
        self.cache.insert(key.to_string(), data);
        Ok(())
    }

    pub fn clear_cache(&mut self) -> Result<(), ExtensionError> {
        self.cache.clear();
        Ok(())
    }
}
