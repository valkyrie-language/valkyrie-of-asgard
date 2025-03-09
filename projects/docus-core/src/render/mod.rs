pub use self::article::ArticleTemplate;
use crate::{
    config::{ArticleConfig, DocusConfig},
    helpers::find_all_books,
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

    let mut config = DocusConfig::load(input)?;
    config.cache_path = cache.to_path_buf();
    // generate css
    config.style.generate_css(output, cache)?;
    // generate books
    let books = find_all_books(input, &config.global)?;
    for book in books {
        tracing::trace!("\n    Book: {}", book.0.display());
        let mut config = config.clone();
        config.book = book.1;
        let output = output.join(&config.book.url);
        build_book(&book.0, &output, cache, config)?;
    }
    Ok(())
}

pub fn build_book(input: &Path, output: &Path, cache: &Path, config: DocusConfig) -> Result<(), DocusError> {
    println!("输出1: {}", output.display());
    let chapters = find_all_chapters(input, &config)?;
    for chapter in chapters {
        tracing::trace!("\n    Chapter: {}", chapter.0.display());
        let mut config = config.clone();
        config.chapter = chapter.1;
        let output = output.join(&config.chapter.url);
        build_chapter(&chapter.0, &output, cache, config)?;
    }

    Ok(())
}
pub fn build_chapter(input: &Path, output: &Path, cache: &Path, mut config: DocusConfig) -> Result<(), DocusError> {
    println!("输出2: {}", output.display());
    create_dir_all(output)?;
    for (path, article) in config.chapter.articles.iter() {
        let input = input.join(path);
        let output = output.join(&config.chapter.url);
        build_article(&input, &output, cache, &article)?
    }
    Ok(())
}

pub fn build_article(input: &Path, output: &Path, _: &Path, config: &ArticleConfig) -> Result<(), DocusError> {
    println!("输入3: {}", input.display());
    println!("输出3: {}", output.display());
    let content = std::fs::read_to_string(input)?;
    let article = ArticleTemplate { article: config, content };
    article.render(&output.with_extension("html"))?;
    Ok(())
}
