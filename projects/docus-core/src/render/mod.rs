pub use self::article::ArticleTemplate;
use crate::{config::RenderConfig, DocusError};
use askama::Template;
use comrak::ComrakOptions;
use std::{fs::create_dir_all, path::Path};

mod article;

pub fn build_site(input: &Path, cache: &Path) -> Result<(), DocusError> {
    let mut config = RenderConfig::load(input)?;
    create_dir_all(cache)?;
    config.cache_path = cache.to_path_buf();
    // generate css
    config.style.generate_css(input, cache)?;
    Ok(())
}

pub fn build_book(root: &Path, cache: &Path, config: RenderConfig) -> Result<(), DocusError> {
    todo!()
}

pub fn build_chapter(root: &Path, cache: &Path, config: RenderConfig) -> Result<(), DocusError> {
    todo!()
}

pub fn build_article(root: &Path, cache: &Path, config: RenderConfig) -> Result<(), DocusError> {
    todo!()
}
