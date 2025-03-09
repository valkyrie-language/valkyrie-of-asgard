use std::path::Path;
use crate::errors::{DocusError, Result};

pub struct SassProcessor {
    options: grass::Options<'static>,
}

impl Default for SassProcessor {
    fn default() -> Self {
        let mut options = grass::Options::default();
        options.style = grass::OutputStyle::Compressed;
        Self { options }
    }
}

impl SassProcessor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn compile_file(&self, path: impl AsRef<Path>) -> Result<String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| DocusError::IoError(e.to_string()))?;
        self.compile_string(&content)
    }

    pub fn compile_string(&self, content: &str) -> Result<String> {
        grass::from_string(content, &self.options)
            .map_err(|e| DocusError::EncodeError {
                format: "sass".to_string(),
                message: e.to_string(),
            })
    }
}