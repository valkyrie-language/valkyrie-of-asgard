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
use std::path::Path;

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
    /// `<article-name>.toml`
    pub article: ArticleConfig,
}

impl RenderConfig {
    pub fn load(root: &Path) -> Result<Self, DocusError> {
        let global = DocusConfig::load(&root.join("docus.toml"))?;
        Ok(Self {
            global,
            sidebar: SidebarConfig {},
            topbar: None,
            book: BookConfig {
                title: "".to_string(),
                description: None,
                chapter_order: vec![],
                template: "".to_string(),
                output_dir: None,
            },
            style: StyleConfig { theme: "".to_string(), variables: Default::default() },
            chapter: ChapterConfig { title: "".to_string(), collapsible: false, collapsed: false, items: vec![], index: None },
            article: ArticleConfig {},
        })
    }
}
