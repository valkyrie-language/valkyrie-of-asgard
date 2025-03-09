use crate::DocusError;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ArticleConfig {
    pub url: String,
}

#[derive(Clone, Debug, Deserialize)]
struct ArticleFile {
    url: Option<String>,
}

impl ArticleConfig {
    pub fn load(path: &Path) -> Result<Self, DocusError> {
        let config = path.with_extension("toml");
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if config.exists() {
            let file = toml::from_str::<ArticleFile>(&std::fs::read_to_string(path).unwrap())?;
            Ok(Self { url: file.url.unwrap_or(file_name.to_string()) })
        }
        else {
            Ok(Self { url: file_name.to_string() })
        }
    }
}
