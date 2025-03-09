#![doc = include_str!("readme.md")]

mod article;
mod book;
mod chapter;
mod i18n;
mod render;
mod sidebar;
mod style;
mod topbar;

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
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

#[derive(Default, Clone, Debug)]
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
        let mut render = Self::default();
        render.global = DocusConfig::load(&root.join("docus.toml"))?;
        render.style = StyleConfig::load(&root.join("style.sass"))?;
        render.find_all_books(&root, &render.global.clone())?;
        Ok(render)
    }

    fn find_all_books(&mut self, root: &Path, global: &DocusConfig) -> Result<(), DocusError> {
        for entry in walkdir::WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
            // All folder with `book.toml` is a book
            if entry.file_name() == "book.toml" {
                match entry.path().parent() {
                    Some(dir) => {
                        let book = BookConfig::load(dir, global)?;
                        self.book.insert(book.url.clone(), book);
                    }
                    None => {}
                }
            }
        }
        Ok(())
    }
}
