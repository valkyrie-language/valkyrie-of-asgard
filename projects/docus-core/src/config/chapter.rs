use crate::{config::ArticleConfig, DocusError};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};
use crate::config::InternationalizationConfig;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChapterConfig {
    
    /// The title of the chapter
    pub title: String,
    /// The url of the chapter
    pub url: String,
    /// The article list in the chapter
    pub articles: IndexMap<String, ArticleConfig>,
    /// Whether the chapter is collapsible
    pub collapsible: bool,
    /// Whether the chapter is collapsed in default
    pub collapsed: bool,
    /// The input folder of the book
    pub input: PathBuf,
    /// The output folder of the book
    pub output: PathBuf,
}

#[derive(Debug, Deserialize)]
struct ChapterFile {
    title: Option<String>,
    url: Option<String>,
    /// The order of articles in the chapter
    articles: Option<Vec<String>>,
    collapsible: Option<bool>,
    collapsed: Option<bool>,
}

impl Default for ChapterConfig {
    fn default() -> Self {
        Self { title: "".to_string(), url: "".to_string(), articles: IndexMap::default(), collapsible: false, collapsed: false }
    }
}

impl ChapterConfig {
    pub fn load(folder: &Path, i18n: &InternationalizationConfig) -> Result<Self, DocusError> {
        let mut output = Self::default();
        let config = folder.join("index.toml");
        if config.exists() {
            let dir_name = folder.file_name().unwrap().to_str().unwrap();
            let file = toml::from_str::<ChapterFile>(&std::fs::read_to_string(config).unwrap())?;
            output.title = file.title.unwrap_or(dir_name.to_string());
            output.url = file.url.unwrap_or(dir_name.to_string());
            output.collapsible = file.collapsible.unwrap_or(false);
            output.collapsed = file.collapsed.unwrap_or(false);
            match file.articles {
                Some(order) => {
                    for article in order {
                        let article_path = folder.join(&article);
                        let article_cfg = ArticleConfig::load(&article_path)?;
                        output.articles.insert(article_cfg.url.clone(), article_cfg);
                    }
                }
                None => output.find_articles(folder)?,
            }
        }
        else {
            output.url = folder.file_name().unwrap().to_str().unwrap().to_string();
            output.find_articles(folder)?
        }
        Ok(output)
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
