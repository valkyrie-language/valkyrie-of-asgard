use super::*;

#[derive(Debug, Template)]
#[template(path = "article.html")]
pub struct ArticleTemplate<'a> {
    pub article: &'a ArticleConfig,
    pub content: String,
}

impl<'a> ArticleTemplate<'a> {
    pub fn render(&self, output: &Path) -> Result<(), DocusError> {

        let mut options = ComrakOptions::default();
        options.extension.table = true;
        options.extension.strikethrough = true;
        options.extension.tasklist = true;
        options.extension.autolink = true;
        options.extension.header_ids = Some(String::new());
        options.render.github_pre_lang = true;
        let content = comrak::markdown_to_html(&self.content, &options);
        let mut file = std::fs::File::create(output)?;
        self.write_into(&mut file)?;
        Ok(())
    }
}
