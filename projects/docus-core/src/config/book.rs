use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BookConfig {
    pub title: String,
    pub description: Option<String>,
    pub chapter_order: Vec<String>,
    #[serde(default)]
    pub template: String,
    #[serde(default)]
    pub output_dir: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct ChapterConfig {
    pub title: String,
    pub collapsible: bool,
    pub collapsed: bool,
    pub items: Vec<NavItem>,
}

#[derive(Debug, Deserialize)]
pub enum NavItem {
    Link { title: String, path: String },
    Group { title: String, items: Vec<String> },
}