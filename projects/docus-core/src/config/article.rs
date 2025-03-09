use crate::DocusError;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ArticleConfig {
    pub name: String,
    pub url: String,
    /// The input name of the article
    pub input: PathBuf,
    /// The output name of the article
    pub output: PathBuf,
}

impl ArticleConfig {
    /// Load the article config from the input file
    pub fn load(input: &Path, output: &Path) -> Result<Self, DocusError> {
        let mut result = Self::default();
        let file_name = input.file_prefix().unwrap().to_str().unwrap();
        result.name = file_name.to_string();
        result.url = file_name.to_string();
        result.input = input.to_path_buf();
        result.output = output.join(&result.url);
        Ok(result)
    }
    /// Read the markdown content of the article
    pub fn read_markdown(&self, language: Option<&String>) -> Result<String, DocusError> {
        println!("X: {}", self.input.with_extension("md").display());
        let content = match language {
            Some(lang) => std::fs::read_to_string(&format!("{}.{}.md", lang, self.input.display())),
            None => std::fs::read_to_string(&self.input.with_extension("md")),
        }?;
        Ok(content)
    }
}
