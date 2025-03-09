use crate::{
    config::{ChapterConfig, DocusConfig, DocusFile, InternationalizationConfig},
    DocusError,
};
use indexmap::IndexMap;
use serde::{Deserialize, Deserializer, Serialize};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BookConfig {
    pub name: String,
    pub url: String,
    pub chapters: IndexMap<String, ChapterConfig>,
    /// The input folder of the book
    pub input: PathBuf,
    /// The output folder of the book
    pub output: PathBuf,
}

#[derive(Clone, Debug, Deserialize)]
struct BookFile {
    name: Option<String>,
    url: Option<String>,
    /// The order of chapters in the book
    chapters: Option<Vec<String>>,
}

impl Default for BookConfig {
    fn default() -> Self {
        Self { name: "".to_string(), url: "".to_string(), chapters: Default::default() }
    }
}

impl BookConfig {
    pub fn load(folder: &Path, global: &DocusConfig) -> Result<Self, DocusError> {
        let mut output = Self::default();
        let config = folder.join("book.toml");
        let dir_name = folder.file_name().unwrap().to_str().unwrap();
        let file = toml::from_str::<BookFile>(&std::fs::read_to_string(config).unwrap())?;
        output.name = file.name.unwrap_or(dir_name.to_string());
        output.url = file.url.unwrap_or(dir_name.to_string());
        match file.chapters {
            Some(order) => output.find_by_order(folder, &order, global),
            None => output.find_in_dir(folder, global)?,
        }
        Ok(output)
    }

    /// find chapters by order
    fn find_by_order(&mut self, folder: &Path, order: &[String], global: &DocusConfig) {
        for chapter in order {
            match ChapterConfig::load(&folder.join(chapter), &global.i18n) {
                Ok(o) => {
                    self.chapters.insert(o.url.clone(), o);
                }
                Err(e) => tracing::error!("{e}"),
            };
        }
    }

    /// find chapter in dir
    fn find_in_dir(&mut self, book: &Path, global: &DocusConfig) -> Result<(), DocusError> {
        for file in book.read_dir()? {
            // all sub folder in book dir are chapter
            match file {
                Ok(o) => match o.file_type() {
                    Ok(ty) => {
                        if ty.is_dir() {
                            let path = o.path();
                            let mut book_cfg = ChapterConfig::load(&path, &global.i18n)?;
                            self.chapters.insert(book_cfg.url.clone(), book_cfg);
                        }
                    }
                    Err(e) => tracing::error!("{e}"),
                },
                Err(e) => tracing::error!("{e}"),
            }
        }
        Ok(())
    }
}
