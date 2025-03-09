use crate::DocusError;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StyleConfig {
    pub main: String,
    pub variables: String,
    pub custom: String,
}

impl Default for StyleConfig {
    fn default() -> Self {
        Self { main: "".to_string(), variables: "".to_string(), custom: "".to_string() }
    }
}

impl StyleConfig {
    pub fn load(path: &Path) -> Result<Self, DocusError> {
        let sass = if path.exists() { std::fs::read_to_string(path)? } else { "".to_string() };
        Ok(Self {
            main: include_str!("../../templates/styles/main.scss").to_string(),
            variables: include_str!("../../templates/styles/_variables.scss").to_string(),
            custom: sass,
        })
    }

    pub fn generate_css(&self, path: &Path, cache: &Path) -> Result<(), DocusError> {
        std::fs::write(cache.join("main.scss"), &self.main)?;
        std::fs::write(cache.join("variables.scss"), &self.variables)?;
        std::fs::write(cache.join("custom.scss"), &self.custom)?;
        let options = grass::Options::default();
        let css = grass::from_path(&cache.join("main.scss"), &options)?;
        let output = path.join("index.css");
        std::fs::write(output, css)?;
        Ok(())
    }
}
