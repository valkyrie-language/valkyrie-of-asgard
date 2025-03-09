pub use self::article::ArticleTemplate;
use crate::{config::RenderConfig, DocusError};
use askama::Template;
use comrak::ComrakOptions;
use std::path::Path;
mod article;

pub fn build_site(root: impl AsRef<Path>, cache: impl AsRef<Path>) -> Result<(), DocusError> {
    todo!()
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
