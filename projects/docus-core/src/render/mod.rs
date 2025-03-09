pub use self::article::ArticleTemplate;
use crate::{
    config::RenderConfig,
    helpers::{find_all_articles, find_all_books, find_all_chapters},
    DocusError,
};
use askama::Template;
use comrak::ComrakOptions;
use std::{fs::create_dir_all, path::Path};

mod article;

pub fn build_site(input: &Path, output: &Path, cache: &Path) -> Result<(), DocusError> {
    tracing::debug!("\n    Input: {}", input.display());
    tracing::debug!("\n    Output: {}", output.display());
    tracing::debug!("\n    Cache: {}", cache.display());

    let mut config = RenderConfig::load(input)?;
    config.cache_path = cache.to_path_buf();
    // generate css
    config.style.generate_css(output, cache)?;
    // generate books
    let books = find_all_books(input)?;
    for book in books {
        tracing::trace!("\n    Book: {}", book.0.display());
        let mut config = config.clone();
        config.book = book.1;
        let output = output.join(&config.book.url);
        build_book(&book.0, &output, cache, config)?;
    }
    Ok(())
}

pub fn build_book(input: &Path, output: &Path, cache: &Path, config: RenderConfig) -> Result<(), DocusError> {
    println!("输出1: {}", output.display());
    let chapters = find_all_chapters(input)?;
    for chapter in chapters {
        tracing::trace!("\n    Chapter: {}", chapter.0.display());
        let mut config = config.clone();
        config.chapter = chapter.1;
        let output = output.join(&config.chapter.url);
        build_chapter(&chapter.0, &output, cache, config)?;
    }

    Ok(())
}
pub fn build_chapter(input: &Path, output: &Path, cache: &Path, mut config: RenderConfig) -> Result<(), DocusError> {
    println!("输出2: {}", output.display());
    create_dir_all(output)?;
    let chapters = find_all_articles(input)?;
    for article in chapters {
        tracing::trace!("\n    Article: {}", article.0.display());
        let mut config = config.clone();
        config.article = article.1;
        build_article(&article.0, output, cache, config)?;
    }
    Ok(())
}

pub fn build_article(input: &Path, output: &Path, _: &Path, config: RenderConfig) -> Result<(), DocusError> {
    let output = output.join(&config.article.url);
    println!("输出3: {}", output.display());
    ArticleTemplate::new(&config, input)?.render(&output.with_extension("html"))?;
    Ok(())
}
