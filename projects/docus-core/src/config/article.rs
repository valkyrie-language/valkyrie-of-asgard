use crate::DocusError;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ArticleConfig {
    pub url: String,
    /// The input name of the article
    pub input: PathBuf,
    /// The output name of the article
    pub output: PathBuf,
}

#[derive(Clone, Debug, Deserialize)]
struct ArticleFile {
    url: Option<String>,
}

impl ArticleConfig {
    pub fn load(input: &Path, output: &Path) -> Result<Self, DocusError> {
        let mut result = Self::default();
        let config = input.with_extension("toml");
        let file_name = input.file_name().unwrap().to_str().unwrap();
        if config.exists() {
            let file = toml::from_str::<ArticleFile>(&std::fs::read_to_string(input).unwrap())?;
            result.url = file.url.unwrap_or(file_name.to_string());
        }
        else {
            result.url = file_name.to_string();
        }
        result.input = input.to_path_buf();
        result.output = output.join(&result.url);
        Ok(result)
    }
}
