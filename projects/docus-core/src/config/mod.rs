mod article;
mod book;
mod chapter;
mod i18n;
mod render;
mod sidebar;
mod topbar;

pub use self::{
    book::BookConfig,
    chapter::ChapterConfig,
    i18n::{InternationalizationConfig, LanguageConfig},
    render::DocusConfig,
};
use crate::config::{article::ArticleConfig, sidebar::SidebarConfig, topbar::TopbarConfig};

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
    pub style: String,
    /// `chapter.toml`
    pub chapter: ChapterConfig,
    /// `<article-name>.toml`
    pub article: ArticleConfig,
}
