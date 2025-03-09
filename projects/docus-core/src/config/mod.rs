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
    pub book: BookConfig,
    /// `style.sass`
    pub style: StyleConfig,
    /// `chapter.toml`
    pub chapter: ChapterConfig,
    /// The path to the cache directory
    pub cache_path: PathBuf
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
            chapter: ChapterConfig::default(),
            cache_path: Default::default(),
        })
    }
}
