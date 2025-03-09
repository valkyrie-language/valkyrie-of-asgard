mod book;
mod chapter;
mod global;
mod i18n;

pub use self::{
    book::BookConfig,
    chapter::ChapterConfig,
    global::GlobalConfig,
    i18n::{InternationalizationConfig, LanguageConfig},
};
