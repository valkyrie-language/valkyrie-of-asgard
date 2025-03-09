use std::path::PathBuf;
use std::fs;

use crate::config::{DocusConfig, SidebarConfig, TopbarConfig};
use crate::markdown::MarkdownRenderer;

pub struct StaticSiteGenerator {
    config: DocusConfig,
    sidebar: SidebarConfig,
    topbar: TopbarConfig,
    renderer: MarkdownRenderer,
}

impl StaticSiteGenerator {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = DocusConfig::load()?;
        let sidebar = SidebarConfig::load()?;
        let topbar = TopbarConfig::load()?;
        let renderer = MarkdownRenderer::new();

        Ok(Self {
            config,
            sidebar,
            topbar,
            renderer,
        })
    }

    pub fn generate(&self, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
        // 创建输出目录
        fs::create_dir_all(output_dir)?;

        // 生成首页
        let index_content = fs::read_to_string("docs/index.md")
            .unwrap_or_else(|_| "# Welcome\n\nNo index page found.".to_string());
        let index_html = self.renderer.render_string(&index_content);
        fs::write(
            PathBuf::from(output_dir).join("index.html"),
            index_html
        )?;

        // TODO: 实现其他页面的生成
        // 1. 遍历 docs 目录下的所有 .md 文件
        // 2. 为每个文件生成对应的 HTML
        // 3. 保持目录结构
        // 4. 应用主题和模板

        Ok(())
    }
}