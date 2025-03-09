use crate::{
    config::{ArticleConfig, InternationalizationConfig},
    DocusError,
};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ChapterConfig {
    /// The title of the chapter
    pub name: String,
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
    articles: Option<Vec<ArticleItem>>,
    collapsible: Option<bool>,
    collapsed: Option<bool>,
}
#[derive(Debug, Deserialize)]
pub struct ArticleItem {
    /// Article name show on the sidebar
    name: Option<String>,
    /// Article local path of the book
    path: String,
    /// Article url of the book
    url: Option<String>,
}

impl ChapterConfig {
    pub fn load(input: &Path, output: &Path, i18n: &InternationalizationConfig) -> Result<Self, DocusError> {
        let mut result = Self::default();
        let config = input.join("index.toml");
        result.input = input.to_path_buf();
        if config.exists() {
            let dir_name = input.file_name().unwrap().to_str().unwrap();
            let file = toml::from_str::<ChapterFile>(&std::fs::read_to_string(config).unwrap())?;
            result.name = file.title.unwrap_or(dir_name.to_string());
            result.url = file.url.unwrap_or(dir_name.to_string());
            result.output = output.join(&result.url);
            result.collapsible = file.collapsible.unwrap_or(false);
            result.collapsed = file.collapsed.unwrap_or(false);
            match file.articles {
                Some(order) => result.find_by_ids(order)?,
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

    fn find_by_ids(&mut self, order: Vec<ArticleItem>) -> Result<(), DocusError> {
        for article in order.into_iter() {
            let article_path = self.input.join(&article.path);
            let mut config = ArticleConfig::load(&article_path, &self.output)?;
            match article.url {
                Some(s) => config.url = s,
                None => {}
            }
            match article.name {
                Some(s) => config.name = s,
                None => {}
            }
            self.articles.insert(config.url.clone(), config);
        }
        Ok(())
    }

    /// Find all article in the chapter folder
    fn find_by_dir(&mut self, folder: &Path) -> Result<(), DocusError> {
        for file in folder.read_dir()? {
            // all markdown files are articles
            if let Ok(file) = file {
                let file_name = file.file_name();
                if file_name.to_string_lossy().ends_with(".md") {
                    let book_cfg = ArticleConfig::load(&file.path(), &self.output)?;
                    self.articles.insert(file_name.to_string_lossy().to_string(), book_cfg);
                }
            }
        }
        Ok(())
    }
}
