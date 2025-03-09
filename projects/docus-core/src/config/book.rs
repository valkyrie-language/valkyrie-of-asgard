use crate::DocusError;
use serde::{Deserialize, Deserializer, Serialize};
use std::path::Path;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BookConfig {
    pub name: String,
    pub url: String,
    pub description: Option<String>,
    pub chapter_order: Vec<String>,
    #[serde(default)]
    pub template: String,
}

#[derive(Clone, Debug, Deserialize)]
struct BookFile {
    name: Option<String>,
    url: Option<String>,
}

impl BookConfig {
    pub fn load(path: &Path) -> Result<Self, DocusError> {
        let config = path.join("book.toml");
        let dir_name = path.file_name().unwrap().to_str().unwrap();
        let file = toml::from_str::<BookFile>(&std::fs::read_to_string(config).unwrap())?;
        Ok(Self {
            name: file.name.unwrap_or(dir_name.to_string()),
            url: file.url.unwrap_or(dir_name.to_string()),
            description: None,
            chapter_order: vec![],
            template: "".to_string(),
        })
    }
}
