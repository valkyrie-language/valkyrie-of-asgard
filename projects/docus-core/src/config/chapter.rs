use crate::{
    config::{ArticleConfig, InternationalizationConfig},
    DocusError,
};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

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
        Self {
            title: "".to_string(),
            url: "".to_string(),
            articles: IndexMap::default(),
            collapsible: false,
            collapsed: false,
            input: Default::default(),
            output: Default::default(),
        }
    }
}

impl ChapterConfig {
    pub fn load(input: &Path, output: &Path, i18n: &InternationalizationConfig) -> Result<Self, DocusError> {
        let mut result = Self::default();
        let config = input.join("index.toml");
        result.input = input.to_path_buf();
        if config.exists() {
            let dir_name = input.file_name().unwrap().to_str().unwrap();
            let file = toml::from_str::<ChapterFile>(&std::fs::read_to_string(config).unwrap())?;
            result.title = file.title.unwrap_or(dir_name.to_string());
            result.url = file.url.unwrap_or(dir_name.to_string());
            result.output = output.join(&result.url);
            result.collapsible = file.collapsible.unwrap_or(false);
            result.collapsed = file.collapsed.unwrap_or(false);
            match file.articles {
                Some(order) => result.find_by_ids(&order)?,
                None => result.find_by_dir(input)?,
            }
        }
        else {
            result.url = input.file_name().unwrap().to_str().unwrap().to_string();
            result.output = output.join(&result.url);
            result.find_by_dir(input)?
        }

        Ok(result)
    }

    fn find_by_ids(&mut self, order: &[String]) -> Result<(), DocusError> {
        for article in order {
            let article_path = self.input.join(article);
            let article_cfg = ArticleConfig::load(&article_path, &self.output)?;
            self.articles.insert(article_cfg.url.clone(), article_cfg);
        }
        Ok(())
    }

    /// Find all article in the chapter folder
    fn find_by_dir(&mut self, folder: &Path) -> Result<(), DocusError> {
        for file in folder.read_dir()? {
            // all markdown files are articles
            if let Ok(file) = file {
                let file_name = file.file_name();
                if file_name.to_string_lossy().eq("index.md") {
                    continue;
                }
                if file_name.to_string_lossy().ends_with(".md") {
                    let book_cfg = ArticleConfig::load(&file.path(), &self.output)?;
                    self.articles.insert(file_name.to_string_lossy().to_string(), book_cfg);
                }
            }
        }
        Ok(())
    }
}
