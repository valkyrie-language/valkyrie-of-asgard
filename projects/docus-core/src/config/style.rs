use crate::DocusError;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StyleConfig {
    pub custom: String,
}

impl Default for StyleConfig {
    fn default() -> Self {
        Self { custom: "".to_string() }
    }
}

impl StyleConfig {
    pub fn load(path: &Path) -> Result<Self, DocusError> {
        let sass = if path.exists() { std::fs::read_to_string(path)? } else { "".to_string() };
        Ok(Self { custom: sass })
    }

    pub fn generate_css(&self, path: &Path) -> Result<(), DocusError> {
        let options = grass::Options::default();
        let css = grass::from_path(&self.custom, &options)?;
        let file = path.join("style.css");
        std::fs::write(file, css)?;
        Ok(())
    }
}
