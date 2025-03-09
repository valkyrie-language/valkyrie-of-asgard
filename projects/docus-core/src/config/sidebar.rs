use crate::{
    config::{BookConfig, ChapterConfig},
    DocusError,
};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct SidebarConfig {
    pub menu: Vec<MenuItem>,
}
#[derive(Deserialize)]
struct SidebarFile {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MenuItem {
    pub name: String,
    pub url: String,
    #[serde(default)]
    pub children: Vec<MenuItem>,
    #[serde(default)]
    pub collapsible: bool,
    #[serde(default)]
    pub collapsed: bool,
}

impl SidebarConfig {
    pub fn load(book: &Path) -> Result<Self, DocusError> {
        let file = book.join("sidebar.toml");
        if file.exists() {
            let _ = toml::from_str::<SidebarFile>(&std::fs::read_to_string(file)?)?;
            Ok(Self { menu: vec![] })
        }
        else {
            Ok(Self { menu: vec![] })
        }
    }

    /// Build the menu from the book
    pub fn build_from_book(book: &BookConfig) -> Self {
        let mut menu = Vec::new();
        for chapter in book.chapters.values() {
            menu.push(Self::build_menu_item(chapter, book));
        }
        Self { menu }
    }

    fn build_menu_item(chapter: &ChapterConfig, book: &BookConfig) -> MenuItem {
        let mut items = Vec::new();
        for article in chapter.articles.values() {
            items.push(MenuItem {
                name: article.name.clone(),
                url: match book.url.as_str() {
                    "" => format!("/{}/{}", chapter.url, article.url),
                    _ => format!("/{}/{}/{}", book.url, chapter.url, article.url),
                },
                children: Vec::new(),
                collapsible: false,
                collapsed: false,
            });
        }
        MenuItem {
            name: chapter.name.clone(),
            url: match book.url.as_str() {
                "" => format!("/{}", chapter.url),
                _ => format!("/{}/{}", book.url, chapter.url),
            },
            children: items,
            collapsible: chapter.collapsible,
            collapsed: chapter.collapsed,
        }
    }
}
