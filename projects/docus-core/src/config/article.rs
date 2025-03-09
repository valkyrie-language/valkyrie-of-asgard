use crate::DocusError;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ArticleConfig {
    url: String,
}

#[derive(Clone, Debug, Deserialize)]
struct ArticleFile {
    url: Option<String>,
}

impl ArticleConfig {
    pub fn load(path: &Path) -> Result<Self, DocusError> {
        let dir_name = path.file_name().unwrap().to_str().unwrap();
        let file = toml::from_str::<ArticleFile>(&std::fs::read_to_string(path).unwrap())?;
        Ok(Self { url: file.url.unwrap_or(dir_name.to_string()) })
    }
}
