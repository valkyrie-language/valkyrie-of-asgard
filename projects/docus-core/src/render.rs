use pulldown_cmark::{html, Options, Parser};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MarkdownError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub struct MarkdownRenderer {
    options: Options,
}

impl Default for MarkdownRenderer {
    fn default() -> Self {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);
        Self { options }
    }
}

impl MarkdownRenderer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render_file(&self, path: impl AsRef<Path>) -> Result<String, MarkdownError> {
        let content = std::fs::read_to_string(path)?;
        Ok(self.render_string(&content))
    }

    pub fn render_string(&self, content: &str) -> String {
        let parser = Parser::new_ext(content, self.options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    }
}
