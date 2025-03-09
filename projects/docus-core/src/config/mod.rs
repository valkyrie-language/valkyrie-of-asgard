#![doc = include_str!("readme.md")]

mod article;
mod book;
mod chapter;
mod i18n;
mod render;
mod sidebar;
mod style;
mod topbar;

use std::collections::BTreeMap;
pub use self::{
    article::ArticleConfig,
    book::BookConfig,
    chapter::ChapterConfig,
    i18n::{InternationalizationConfig, LanguageConfig},
    render::DocusConfig,
    sidebar::SidebarConfig,
    style::StyleConfig,
    topbar::TopbarConfig,
};
use crate::DocusError;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct RenderConfig {
    /// `docus.toml`
    pub global: DocusConfig,
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
}

impl RenderConfig {
    pub fn load(root: &Path) -> Result<Self, DocusError> {
        let global = DocusConfig::load(&root.join("docus.toml"))?;
        let style = StyleConfig::load(&root.join("style.sass"))?;
        Ok(Self {
            global,
            sidebar: SidebarConfig {},
            topbar: None,
            book: BookConfig::default(),
            style,
            cache_path: Default::default(),
        })
    }
}

pub fn find_all_books(root: &Path, global: &DocusConfig) -> Result<Vec<(PathBuf, BookConfig)>, DocusError> {
    let mut results = vec![];
    for entry in walkdir::WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_name() == "book.toml" {
            match entry.path().parent() {
                Some(dir) => {
                    results.push((dir.to_path_buf(), BookConfig::load(dir, global)?));
                }
                None => tracing::error!("{:?}", entry.path()),
            }
        }
    }
    Ok(results)
}

