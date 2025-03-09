use crate::{
    config::{DocusConfig, SidebarConfig, TopbarConfig},
    errors::{DocusError, Result},
};
use askama::Template;
use comrak::{markdown_to_html, ComrakOptions};
use std::path::Path;

#[derive(Template)]
#[template(path = "page.html")]
pub struct PageTemplate<'a> {
    pub title: &'a str,
    pub content: String,
    pub config: &'a RenderContext,
}

pub struct RenderContext {
    pub config: DocusConfig,
    pub sidebar: SidebarConfig,
    pub topbar: TopbarConfig,
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

    pub fn render_file(&self, path: impl AsRef<Path>, title: &str, config: &RenderContext, lang: &str) -> Result<String> {
        todo!()
    }

    pub fn render_string(&self, content: &str, title: &str, config: &RenderContext) -> Result<String> {
        let html_output = markdown_to_html(content, &self.options);
        let template = PageTemplate { title, content: html_output, config };
        template.render().map_err(|e| DocusError::RenderError { path: String::new(), message: e.to_string() })
    }
}
