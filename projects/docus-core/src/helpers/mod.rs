use crate::{
    config::{ArticleConfig, BookConfig, ChapterConfig},
    DocusError,
};
use std::path::{Path, PathBuf};

pub fn find_all_books(root: &Path) -> Result<Vec<(PathBuf, BookConfig)>, DocusError> {
    let mut results = vec![];
    for entry in walkdir::WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_name() == "book.toml" {
            match entry.path().parent() {
                Some(dir) => {
                    results.push((dir.to_path_buf(), BookConfig::load(dir)?));
                }
                None => tracing::error!("{:?}", entry.path()),
            }
        }
    }
    Ok(results)
}

pub fn find_all_chapters(root: &Path) -> Result<Vec<(PathBuf, ChapterConfig)>, DocusError> {
    let mut results = vec![];
    for file in root.read_dir()? {
        // all `index.toml` are chapters
        match file {
            Ok(o) => match o.file_type() {
                Ok(ty) => {
                    if ty.is_dir() {
                        let path = o.path();
                        let book_cfg = ChapterConfig::load(&path)?;
                        results.push((path, book_cfg));
                    }
                }
                Err(e) => tracing::error!("{e}"),
            },
            Err(e) => tracing::error!("{e}"),
        }
    }
    Ok(results)
}

pub fn find_all_articles(root: &Path) -> Result<Vec<(PathBuf, ArticleConfig)>, DocusError> {
    let mut results = vec![];
    for file in root.read_dir()? {
        // all markdown files are articles
        if let Ok(file) = file {
            let file_name = file.file_name();
            if file_name.to_string_lossy().eq("index.md") {
                continue;
            }
            if file_name.to_string_lossy().ends_with(".md") {
                let book_cfg = ArticleConfig::load(&file.path())?;
                results.push((file.path().to_path_buf(), book_cfg));
            }
        }
    }
    Ok(results)
}
