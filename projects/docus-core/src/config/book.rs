use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct BookConfig {
    pub title: String,
    pub description: Option<String>,
    pub chapter_order: Vec<String>,
    #[serde(default)]
    pub template: String,
    #[serde(default)]
    pub output_dir: Option<String>,
}

