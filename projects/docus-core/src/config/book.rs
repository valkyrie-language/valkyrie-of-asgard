use crate::{
    config::{ChapterConfig, DocusConfig, DocusFile, InternationalizationConfig, SidebarConfig},
    DocusError,
};
use indexmap::IndexMap;
use serde::{Deserialize, Deserializer, Serialize};
use std::path::{Path, PathBuf};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct BookConfig {
    pub name: String,
    pub url: String,
    pub chapters: IndexMap<String, ChapterConfig>,
    pub sidebar: SidebarConfig,
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

impl BookConfig {
    pub fn load(input: &Path, global: &DocusConfig) -> Result<Self, DocusError> {
        let mut output = Self::default();
        let config = input.join("book.toml");
        let dir_name = input.file_name().unwrap().to_str().unwrap();
        let file = toml::from_str::<BookFile>(&std::fs::read_to_string(config).unwrap())?;
        output.name = file.name.unwrap_or(dir_name.to_string());
        output.url = file.url.unwrap_or(dir_name.to_string());
        output.input = input.to_path_buf();
        output.output = global.output_path.join(&output.url);
        match file.chapters {
            Some(order) => output.find_by_order(&order, global),
            None => output.find_in_dir(global)?,
        }
        output.sidebar = SidebarConfig::build_from_book(&output);
        Ok(output)
    }

    /// find chapters by order
    fn find_by_order(&mut self, order: &[String], global: &DocusConfig) {
        for chapter in order {
            match ChapterConfig::load(&self.input.join(chapter), &self.output, &global.i18n) {
                Ok(o) => {
                    self.chapters.insert(o.url.clone(), o);
                }
                Err(e) => tracing::error!("{e}"),
            };
        }
    }

    /// find chapter in dir
    fn find_in_dir(&mut self, global: &DocusConfig) -> Result<(), DocusError> {
        for file in self.input.read_dir()? {
            // all sub folder in book dir are chapter
            match file {
                Ok(o) => match o.file_type() {
                    Ok(ty) => {
                        if ty.is_dir() {
                            let mut book_cfg = ChapterConfig::load(&o.path(), &self.output, &global.i18n)?;
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
