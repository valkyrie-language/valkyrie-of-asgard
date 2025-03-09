use crate::config::InternationalizationConfig;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GlobalConfig {
    pub output_dir: String,
    #[serde(default)]
    pub style: StyleConfig,
    #[serde(default)]
    pub i18n: InternationalizationConfig,
}

#[derive(Debug, Deserialize)]
pub struct LanguageConfig {
    pub code: String,
    pub name: String,
    #[serde(default)]
    pub fallback: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StyleConfig {
    #[serde(default)]
    pub theme: String,
    #[serde(default)]
    pub variables: std::collections::HashMap<String, String>,
}

impl Default for StyleConfig {
    fn default() -> Self {
        Self { theme: "light".to_string(), variables: Default::default() }
    }
}
