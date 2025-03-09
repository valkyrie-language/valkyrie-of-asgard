use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("TOML parse error: {0}")]
    Toml(#[from] toml::de::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocusConfig {
    pub title: String,
    pub description: Option<String>,
    pub base_url: Option<String>,
    pub theme: Option<String>,
    pub language: Option<String>,
    pub output_dir: Option<PathBuf>,
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
    pub fn load() -> Result<Self, ConfigError> {
        let config_path = ".config/docus.toml";
        let content = std::fs::read_to_string(config_path)?;
        let config: DocusConfig = toml::from_str(&content)?;
        Ok(config)
    }
}

impl SidebarConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let config_path = "docs/sidebar.toml";
        let content = std::fs::read_to_string(config_path)?;
        let config: SidebarConfig = toml::from_str(&content)?;
        Ok(config)
    }
}

impl TopbarConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let config_path = "docs/topbar.toml";
        let content = std::fs::read_to_string(config_path)?;
        let config: TopbarConfig = toml::from_str(&content)?;
        Ok(config)
    }
}