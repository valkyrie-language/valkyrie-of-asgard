use crate::{config::ArticleConfig, DocusError};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ChapterConfig {
    pub title: String,
    pub url: String,
    pub articles: BTreeMap<String, ArticleConfig>,
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
    pub fn load(folder: &Path) -> Result<Self, DocusError> {
        let config = folder.join("index.toml");
        if config.exists() {
            let dir_name = folder.file_name().unwrap().to_str().unwrap();
            let file = toml::from_str::<ChapterFile>(&std::fs::read_to_string(config).unwrap())?;
            Ok(Self {
                title: "".to_string(),
                url: file.url.unwrap_or(dir_name.to_string()),
                articles: BTreeMap::default(),
                collapsible: false,
                collapsed: false,
            })
        }
        else {
            Ok(Self {
                title: "".to_string(),
                url: folder.file_name().unwrap().to_str().unwrap().to_string(),
                articles: BTreeMap::default(),
                collapsible: false,
                collapsed: false,
            })
        }
    }
    /// Find all article in the chapter folder
    pub fn find_articles(&mut self, folder: &Path) -> Result<(), DocusError> {
        let chapters = find_all_articles(folder)?;
        for (article, config) in chapters {
            tracing::trace!("\n    Article: {}", article.display());
            self.articles.insert(config.url.clone(), config);
        }
        Ok(())
    }
}

pub fn find_all_articles(root: &Path) -> Result<Vec<(PathBuf, ArticleConfig)>, DocusError> {
    let mut results = vec![];
    for file in root.read_dir()? {
        // all markdown files are articles
        if let Ok(file) = file {
            let file_name = file.file_name();
            if file_name.to_string_lossy().eq("index.md") {
                continue;
            }
            if file_name.to_string_lossy().ends_with(".md") {
                let book_cfg = ArticleConfig::load(&file.path())?;
                results.push((file.path().to_path_buf(), book_cfg));
            }
        }
    }
    Ok(results)
}
