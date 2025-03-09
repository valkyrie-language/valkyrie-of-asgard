use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ChapterConfig {
    pub title: String,
    #[serde(default)]
    pub collapsible: bool,
    #[serde(default)]
    pub collapsed: bool,
    pub items: Vec<NavItem>,
    #[serde(default)]
    pub index: Option<String>,
}

#[derive(Debug, Deserialize)]
pub enum NavItem {
    Link { 
        title: String,
        path: String,
        #[serde(default)]
        external: bool 
    },
    Group {
        title: String,
        #[serde(default)]
        collapsible: bool,
        #[serde(default)]
        collapsed: bool,
        items: Vec<NavItem>
    }
}

impl ChapterConfig {
    pub fn load(path: &str) -> Result<Self, crate::error::DocusError> {
        let content = std::fs::read_to_string(path)?;
        toml::from_str(&content).map_err(|e| e.into())
    }
}