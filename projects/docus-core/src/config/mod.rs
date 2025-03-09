#![doc = include_str!("readme.md")]

mod article;
mod book;
mod chapter;
mod i18n;
mod sidebar;
mod style;
mod topbar;

pub use self::{
    article::ArticleConfig,
    book::BookConfig,
    chapter::ChapterConfig,
    i18n::{InternationalizationConfig, LanguageConfig},
    sidebar::SidebarConfig,
    style::StyleConfig,
    topbar::TopbarConfig,
};
use crate::DocusError;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

#[derive(Default, Clone, Debug)]
pub struct DocusConfig {
    /// `sidebar.toml`
    pub sidebar: SidebarConfig,
    /// `topbar.toml`
    pub topbar: Option<TopbarConfig>,
    /// `book.toml`
    pub book: BTreeMap<String, BookConfig>,
    /// `style.sass`
    pub style: StyleConfig,
    /// The path to the cache directory
    pub cache_path: PathBuf,
    pub i18n: InternationalizationConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct DocusFile {
    output: Option<String>,
    i18n: Option<InternationalizationConfig>,
}

impl DocusConfig {
    pub fn load(root: &Path) -> Result<Self, DocusError> {
        let mut render = Self::default();
        let file = toml::from_str::<DocusFile>(&std::fs::read_to_string(root.join("docus.toml"))?)?;
        render.i18n = file.i18n.unwrap_or_default();
        render.style = StyleConfig::load(&root.join("style.sass"))?;
        render.find_all_books(&root)?;
        Ok(render)
    }

    fn find_all_books(&mut self, root: &Path) -> Result<(), DocusError> {
        for entry in walkdir::WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
            // All folder with `book.toml` is a book
            if entry.file_name() == "book.toml" {
                match entry.path().parent() {
                    Some(dir) => {
                        let book = BookConfig::load(dir, self)?;
                        self.book.insert(book.url.clone(), book);
                    }
                    None => {}
                }
            }
        }
        Ok(())
    }
}
