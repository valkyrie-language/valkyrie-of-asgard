use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GlobalConfig {
    pub output_dir: String,
    #[serde(default)]
    pub i18n: I18nConfig,
    #[serde(default)]
    pub style: StyleConfig,
}

#[derive(Debug, Deserialize, Default)]
pub struct I18nConfig {
    pub default_lang: String,
    #[serde(default)]
    pub languages: Vec<LanguageConfig>,
}

#[derive(Debug, Deserialize)]
pub struct LanguageConfig {
    pub code: String,
    pub name: String,
    #[serde(default)]
    pub fallback: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct StyleConfig {
    #[serde(default)]
    pub theme: String,
    #[serde(default)]
    pub variables: std::collections::HashMap<String, String>,
}