pub use self::article::ArticleTemplate;
use crate::{config::RenderConfig, DocusError};
use askama::Template;
use comrak::ComrakOptions;
use std::path::Path;
mod article;

pub fn build_site(root: impl AsRef<Path>) -> Result<(), DocusError> {
    todo!()
}

pub fn build_book(root: impl AsRef<Path>, config: RenderConfig) -> Result<(), DocusError> {
    todo!()
}

pub fn build_chapter(root: impl AsRef<Path>, config: RenderConfig) -> Result<(), DocusError> {
    todo!()
}

pub fn build_article(root: impl AsRef<Path>, config: RenderConfig) -> Result<(), DocusError> {
    todo!()
}
