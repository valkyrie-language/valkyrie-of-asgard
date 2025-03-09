pub mod global;
pub mod book;
pub mod chapter;

#[derive(Debug, thiserror::Error)]
pub enum DocusError {
    #[error("IO error: {message}")]
    IoError { path: String, message: String },
    #[error("Config error: {0}")]
    ConfigError(String),
}
