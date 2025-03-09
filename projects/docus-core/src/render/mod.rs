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

pub fn build_book(root: &Path, cache: &Path, mut config: RenderConfig) -> Result<(), DocusError> {
    let book_configs = find_book_configs(root)?;
    
    for (book_path, book_cfg) in book_configs {
        let mut temp_config = config.clone();
        temp_config.book = book_cfg;
        
        let output_dir = cache.join("books").join(&temp_config.book.title);
        std::fs::create_dir_all(&output_dir)?;
        
        // 实际构建逻辑
        build_single_book(&book_path, &output_dir, &temp_config)?;
    }
    Ok(())
}

fn find_book_configs(root: &Path) -> Result<Vec<(PathBuf, BookConfig)>, DocusError> {
    let mut results = vec![];
    
    for entry in walkdir::WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_name() == "book.toml" {
            let book_cfg = BookConfig::load(&entry.path())?;
            results.push((entry.path().to_path_buf(), book_cfg));
        }
    }
    
    Ok(results)
}

fn build_single_book(root: &Path, output: &Path, config: &RenderConfig) -> Result<(), DocusError> {
    // 递归处理所有章节
    for chapter_path in &config.book.chapter_order {
        let full_path = root.join(chapter_path);
        let mut chapter_config = config.clone();
        chapter_config.chapter.title = chapter_path.file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        // 创建章节输出目录
        let chapter_output = output.join(&chapter_config.chapter.title);
        create_dir_all(&chapter_output)?;

        // 构建章节内容
        build_chapter(&full_path, &chapter_output, chapter_config)?;
    }
    Ok(())
}

pub fn build_chapter(root: &Path, cache: &Path, mut config: RenderConfig) -> Result<(), DocusError> {
    // 生成章节索引页
    let index_content = format!("# {}\n\n## 文章列表\n", config.chapter.title);
    std::fs::write(cache.join("index.md"), index_content)?;

    // 处理章节内的文章
    for entry in std::fs::read_dir(root)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            let article_config = config.clone();
            build_article(&entry.path(), cache, article_config)?;
        }
    }
    Ok(())
}

pub fn build_article(root: &Path, cache: &Path, mut config: RenderConfig) -> Result<(), DocusError> {
    // 自动选择输出路径格式
    let output_path = if config.book.title.is_empty() {
        cache.join("articles").join(root.file_stem().unwrap_or_default())
    } else {
        cache.join(root.file_stem().unwrap_or_default())
    }.with_extension("html");

    // 创建文章模板并渲染
    let template = ArticleTemplate::new(&config, root)?;
    template.render(&output_path)?;
    Ok(())
}
