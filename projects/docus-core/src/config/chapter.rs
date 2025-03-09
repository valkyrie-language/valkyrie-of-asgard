use crate::DocusError;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ChapterConfig {
    pub title: String,
    pub url: String,
    pub collapsible: bool,
    pub collapsed: bool,
}
#[derive(Clone, Debug, Default, Deserialize)]
struct ChapterFile {
    title: Option<String>,
    url: Option<String>,
    collapsible: Option<bool>,
    collapsed: Option<bool>,
}

impl ChapterConfig {
    pub fn load(dir: &Path) -> Result<Self, DocusError> {
        let config = dir.join("index.toml");
        if config.exists() {
            let dir_name = dir.file_name().unwrap().to_str().unwrap();
            let file = toml::from_str::<ChapterFile>(&std::fs::read_to_string(config).unwrap())?;
            Ok(Self {
                title: "".to_string(),
                url: file.url.unwrap_or(dir_name.to_string()),
                collapsible: false,
                collapsed: false,
            })
        }
        else {
            return Ok(Self {
                title: "".to_string(),
                url: dir.file_name().unwrap().to_str().unwrap().to_string(),
                collapsible: false,
                collapsed: false,
            });
        }
    }
}
