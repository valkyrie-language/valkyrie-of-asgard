use std::path::{Path, PathBuf};
use crate::config::{ArticleConfig, BookConfig};
use crate::DocusError;

pub fn find_all_books(root: &Path) -> Result<Vec<(PathBuf, BookConfig)>, DocusError> {
    let mut results = vec![];

    for entry in walkdir::WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_name() == "book.toml" {
            let book_cfg = BookConfig::load(&entry.path())?;
            results.push((entry.path().to_path_buf(), book_cfg));
        }
    }

    Ok(results)
}

pub fn find_all_chapters(root: &Path) -> Result<Vec<(PathBuf, ArticleConfig)>, DocusError> {
    let mut results = vec![];

    for entry in walkdir::WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_name() == "article.toml" {
            let book_cfg = ArticleConfig::load(&entry.path())?;
            results.push((entry.path().to_path_buf(), book_cfg));
        }
    }

    Ok(results)
}
