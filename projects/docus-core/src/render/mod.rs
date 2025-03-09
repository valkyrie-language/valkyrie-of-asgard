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
    tracing::debug!("\n    Input:  {}\n    Output: {}\n    Cache:  {}", input.display(), output.display(), cache.display());
    let mut config = DocusConfig::load(input, output)?;
    config.cache_path = cache.to_path_buf();
    // generate css
    config.style.generate_css(output, cache)?;
    // generate books
    for book in config.books.values() {
        build_book(&book, &config)?
    }
    Ok(())
}

pub fn build_book(config: &BookConfig, global: &DocusConfig) -> Result<(), DocusError> {
    tracing::trace!("\n    Book: {}\n       -> {}", config.input.display(), config.output.display());
    for chapter in config.chapters.values() {
        build_chapter(&chapter, config, global)?
    }
    Ok(())
}
pub fn build_chapter(config: &ChapterConfig, book: &BookConfig, global: &DocusConfig) -> Result<(), DocusError> {
    tracing::trace!("\n    Chapter: {}\n          -> {}", config.input.display(), config.output.display());
    for article in config.articles.values() {
        create_dir_all(&config.output)?;
        build_article(&article, book, global)?
    }
    Ok(())
}

pub fn build_article(config: &ArticleConfig, book: &BookConfig, global: &DocusConfig) -> Result<(), DocusError> {
    tracing::trace!("\n    Article: {}\n          -> {}", config.input.display(), config.output.display());
    let content = std::fs::read_to_string(config.input.with_extension("md"))?;
    let article = ArticleTemplate { book, article: config, global, content };
    article.render(&config.output.with_extension("html"))?;
    Ok(())
}
