pub use self::article::ArticleTemplate;
use crate::{config::RenderConfig, DocusError};
use askama::Template;
use comrak::ComrakOptions;
use std::path::Path;
mod article;
mod style;

pub fn build_site(root: impl AsRef<Path>, cache_dir: impl AsRef<Path>) -> Result<(), DocusError> {
    let cache = cache_dir.as_ref();
    std::fs::create_dir_all(cache)?;

    // 处理SCSS编译
    let scss_cache = cache.join("scss");
    std::fs::create_dir_all(&scss_cache)?;

    // 复制模板SCSS
    let main_scss = include_str!("../../templates/main.scss");
    std::fs::write(scss_cache.join("main.scss"), main_scss)?;

    // 复制用户SCSS
    let user_scss = root.as_ref().join("style.scss");
    if user_scss.exists() {
        std::fs::copy(&user_scss, scss_cache.join("style.scss"))?;
    }

    // 编译SCSS
    let mut sass = sass_rs::Options::new();
    sass.output_style = sass_rs::OutputStyle::Compressed;

    let combined_scss = format!("@import 'main.scss';\n@import 'style.scss';",);

    let css = sass_rs::compile_string(&combined_scss, sass).map_err(|e| DocusError::ScssError(e.to_string()))?;

    // 输出CSS到缓存目录
    let css_dir = cache.join("css");
    std::fs::create_dir_all(&css_dir)?;
    std::fs::write(css_dir.join("index.css"), css)?;

    // 复制到最终输出目录
    let dist_css = root.as_ref().join("dist").join("index.css");
    std::fs::create_dir_all(dist_css.parent().unwrap())?;
    std::fs::copy(css_dir.join("index.css"), dist_css)?;

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
