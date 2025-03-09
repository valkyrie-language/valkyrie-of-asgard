use crate::errors::{DocusError, Result};
use askama::Template;
use comrak::{markdown_to_html, ComrakOptions};
use std::path::{Path, PathBuf};

#[derive(Template)]
#[template(path = "page.html")]
pub struct PageTemplate<'a> {
    pub title: &'a str,
    pub content: String,
    pub sidebar: &'a str,
    pub topbar: &'a str,
}

pub struct MarkdownRenderer {
    options: ComrakOptions,
}

impl Default for MarkdownRenderer {
    fn default() -> Self {
        let mut options = ComrakOptions::default();
        options.extension.table = true;
        options.extension.strikethrough = true;
        options.extension.tasklist = true;
        options.extension.autolink = true;
        options.extension.header_ids = Some(String::new());
        options.render.github_pre_lang = true;
        Self { options }
    }
}

impl MarkdownRenderer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render_file(&self, path: impl AsRef<Path>, title: &str, sidebar: &str, topbar: &str, lang: &str) -> Result<String> {
        let candidates = self.generate_lang_candidates(path.as_ref(), lang)?;
        let content = candidates.iter()
            .find_map(|p| std::fs::read_to_string(p).ok())
            .ok_or_else(|| DocusError::IoError { path: "".to_string(), message: "No valid language file found".into() })?;
        self.render_string(&content, title, sidebar, topbar)
    }

    fn generate_lang_candidates(&self, path: &Path, lang: &str) -> Result<Vec<PathBuf>> {
        let mut candidates = Vec::new();
        let file_stem = path.file_stem().ok_or(DocusError::IoError { path: "".to_string(), message: "".to_string() })?;
        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");

        // 从配置中获取语言回退链
        let mut fallback_chain = vec![lang];
        let languages = &crate::config::CONFIG.i18n.languages;
        
        // 构建完整回退链
        let mut current_lang = lang;
        while let Some(lang) = languages.iter().find(|l| l.code == current_lang) {
            if let Some(fb) = &lang.fallback {
                fallback_chain.push(fb);
                current_lang = fb;
            } else {
                break;
            }
        }
        
        // 添加默认语言作为最后回退
        fallback_chain.push(&crate::config::CONFIG.i18n.main);

        for lang_code in fallback_chain {
            let mut new_path = path.to_path_buf();
            new_path.set_file_name(format!("{}.{}", file_stem.to_str().unwrap(), lang_code));
            if extension != "" {
                new_path.set_extension(extension);
            }
            candidates.push(new_path);
        }
        candidates.push(path.to_path_buf());
        Ok(candidates)
    }

    pub fn render_string(&self, content: &str, title: &str, sidebar: &str, topbar: &str) -> Result<String> {
        let html_output = markdown_to_html(content, &self.options);

        let template = PageTemplate { title, content: html_output, sidebar, topbar };

        template.render().map_err(|e| DocusError::RenderError { path: String::new(), message: e.to_string() })
    }
}
