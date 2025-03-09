pub use self::article::ArticleTemplate;
use crate::{
    config::RenderConfig,
    helpers::{find_all_articles, find_all_books, find_all_chapters},
    DocusError,
};
use askama::Template;
use comrak::ComrakOptions;
use std::path::Path;

mod article;

pub fn build_site(input: &Path, output: &Path, cache: &Path) -> Result<(), DocusError> {
    tracing::trace!("input: {}", input.display());

    let mut config = RenderConfig::load(input)?;
    config.cache_path = cache.to_path_buf();
    // generate css
    config.style.generate_css(output, cache)?;
    // generate books
    let books = find_all_books(input)?;
    for book in books {
        tracing::trace!("book: {}", book.0.display());
        let mut config = config.clone();
        config.book = book.1;
        let output = output.join(&config.book.url);
        build_book(&book.0, &output, cache, config)?;
    }
    Ok(())
}

pub fn build_book(input: &Path, output: &Path, cache: &Path, config: RenderConfig) -> Result<(), DocusError> {
    let chapters = find_all_chapters(input)?;
    for chapter in chapters {
        tracing::trace!("chapter: {}", chapter.0.display());
        let mut config = config.clone();
        config.chapter = chapter.1;
        let output = output.join(&config.chapter.url);
        build_book(&chapter.0, &output, cache, config)?;
    }

    Ok(())
}
pub fn build_chapter(input: &Path, output: &Path, cache: &Path, mut config: RenderConfig) -> Result<(), DocusError> {
    let chapters = find_all_articles(input)?;
    for article in chapters {
        tracing::trace!("article: {}", article.0.display());
        let mut config = config.clone();
        config.article = article.1;
        let output = output.join(&config.article.url);
        build_article(&article.0, cache, config)?;
    }
    Ok(())
}

pub fn build_article(root: &Path, cache: &Path, mut config: RenderConfig) -> Result<(), DocusError> {
    Ok(())
}
