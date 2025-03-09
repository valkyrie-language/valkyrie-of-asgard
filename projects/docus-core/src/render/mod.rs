pub use self::article::ArticleTemplate;
use crate::{
    config::{ArticleConfig, BookConfig, ChapterConfig, DocusConfig, SidebarConfig, TopbarConfig},
    DocusError,
};
use askama::Template;
use comrak::{ComrakOptions, ExtensionOptions};
use std::{fs::create_dir_all, path::Path};

mod article;

pub fn build_site(input: &Path, output: &Path, cache: &Path) -> Result<(), DocusError> {
    tracing::debug!("\n    Input: {}", input.display());
    tracing::debug!("\n    Output: {}", output.display());
    tracing::debug!("\n    Cache: {}", cache.display());

    let mut config = DocusConfig::load(input, output)?;
    config.cache_path = cache.to_path_buf();
    // generate css
    config.style.generate_css(output, cache)?;
    // generate books
    for book in config.books.values() {
        build_book(&book, &config.topbar)?
    }
    Ok(())
}

pub fn build_book(config: &BookConfig, topbar: &TopbarConfig) -> Result<(), DocusError> {
    tracing::trace!("\n    Book: {}\n       -> {}", config.input.display(), config.output.display());
    for chapter in config.chapters.values() {
        build_chapter(&chapter, topbar, &config.sidebar)?
    }
    Ok(())
}
pub fn build_chapter(config: &ChapterConfig, topbar: &TopbarConfig, sidebar: &SidebarConfig) -> Result<(), DocusError> {
    tracing::trace!("\n    Chapter: {}\n          -> {}", config.input.display(), config.output.display());
    for article in config.articles.values() {
        build_article(&article, topbar, sidebar)?
    }
    Ok(())
}

pub fn build_article(config: &ArticleConfig, topbar: &TopbarConfig, sidebar: &SidebarConfig) -> Result<(), DocusError> {
    tracing::trace!("\n    Article: {}\n          -> {}", config.input.display(), config.output.display());
    let content = std::fs::read_to_string(config.input.with_extension("md"))?;
    let article = ArticleTemplate { topbar, article: config, sidebar, content };
    article.render(&config.output.with_extension("html"))?;
    Ok(())
}
