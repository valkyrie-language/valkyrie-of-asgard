pub use crate::config::i18n::InternationalizationConfig;
use crate::DocusError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

mod i18n;

#[derive(Debug, Serialize, Deserialize)]
pub struct DocusConfig {
    pub title: Option<String>,
    pub description: Option<String>,
    pub base_url: Option<String>,
    pub theme: Option<String>,
    pub output_dir: Option<PathBuf>,
    pub i18n: InternationalizationConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SidebarConfig {
    pub items: Vec<SidebarItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SidebarItem {
    pub text: String,
    pub link: Option<String>,
    pub children: Option<Vec<SidebarItem>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopbarConfig {
    pub items: Vec<TopbarItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopbarItem {
    pub text: String,
    pub link: String,
}

impl DocusConfig {
    pub fn load() -> Result<Self, DocusError> {
        let config_path = ".config/docus.toml";
        let content = std::fs::read_to_string(config_path)
            .map_err(|e| DocusError::IoError { path: "".to_string(), message: e.to_string() })?;
        let config: DocusConfig = toml::from_str(&content).map_err(|e| DocusError::ConfigError(e.to_string()))?;
        Ok(config)
    }
}

impl SidebarConfig {
    pub fn load() -> Result<Self, DocusError> {
        let config_path = "docs/sidebar.toml";
        let content = std::fs::read_to_string(config_path)
            .map_err(|e| DocusError::IoError { path: "".to_string(), message: e.to_string() })?;
        let config: SidebarConfig = toml::from_str(&content).map_err(|e| DocusError::ConfigError(e.to_string()))?;
        Ok(config)
    }
}

impl TopbarConfig {
    pub fn load() -> Result<Self, DocusError> {
        let config_path = "docs/topbar.toml";
        let content = std::fs::read_to_string(config_path)
            .map_err(|e| DocusError::IoError { path: "".to_string(), message: e.to_string() })?;
        let config: TopbarConfig = toml::from_str(&content).map_err(|e| DocusError::ConfigError(e.to_string()))?;
        Ok(config)
    }
}
