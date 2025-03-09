use super::*;
use crate::config::{SidebarConfig, TopbarConfig};
use comrak::ExtensionOptions;

#[derive(Debug, Template)]
#[template(path = "article.html")]
pub struct ArticleTemplate<'a> {
    /// The article config
    pub article: &'a ArticleConfig,
    pub sidebar: &'a SidebarConfig,
    /// The topbar navigation items
    pub topbar: &'a TopbarConfig,
    /// The raw content of the article
    pub content: String,
}

impl<'a> ArticleTemplate<'a> {
    /// Render the article template to the output file
    pub fn render(&self, output: &Path) -> Result<(), DocusError> {
        let mut file = std::fs::File::create(output)?;
        self.write_into(&mut file)?;
        Ok(())
    }

    /// Generate the html content of the article
    pub fn generate_html(&self) -> String {
        let mut options = ComrakOptions {
            parse: Default::default(),
            render: Default::default(),
            extension: ExtensionOptions {
                strikethrough: true,
                tagfilter: false,
                table: true,
                autolink: false,
                tasklist: true,
                superscript: false,
                header_ids: None,
                footnotes: false,
                description_lists: false,
                front_matter_delimiter: None,
                multiline_block_quotes: false,
                alerts: false,
                math_dollars: true,
                math_code: true,
                wikilinks_title_after_pipe: false,
                wikilinks_title_before_pipe: false,
                underline: true,
                subscript: false,
                spoiler: false,
                greentext: false,
                image_url_rewriter: None,
                link_url_rewriter: None,
            },
        };
        options.render.github_pre_lang = true;
        comrak::markdown_to_html(&self.content, &options)
    }
}
