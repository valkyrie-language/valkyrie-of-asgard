pub use self::article::ArticleTemplate;
use crate::{config::RenderConfig, helpers::find_all_books, DocusError};
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
        let mut config = config.clone();
        config.book = book.1;
        let output = output.join(&config.book.url);
        build_book(&book.0, &output, cache, config)?;
    }
    Ok(())
}

pub fn build_book(input: &Path, output: &Path, cache: &Path, config: RenderConfig) -> Result<(), DocusError> {
    Ok(())
}
pub fn build_chapter(root: &Path, cache: &Path, mut config: RenderConfig) -> Result<(), DocusError> {
    Ok(())
}

pub fn build_article(root: &Path, cache: &Path, mut config: RenderConfig) -> Result<(), DocusError> {
    Ok(())
}
