use crate::{config::InternationalizationConfig, DocusError};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DocusConfig {
    pub output_dir: Option<String>,
    #[serde(default)]
    pub i18n: InternationalizationConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct DocusFile {
    output: Option<String>,
    i18n: InternationalizationConfig,
}

impl DocusConfig {
    pub fn load(path: &Path) -> Result<Self, DocusError> {
        let config = toml::from_str::<DocusConfig>(&std::fs::read_to_string(path)?)?;
        Ok(config)
    }
}
