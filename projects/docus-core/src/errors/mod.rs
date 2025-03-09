use std::fmt::{Display, Formatter};

mod for_grass;
mod for_std;
mod for_toml;

pub type Result<T> = std::result::Result<T, DocusError>;

#[derive(Debug, Clone)]
pub enum DocusError {
    RenderError { path: String, message: String },
    EncodeError { format: String, message: String },
    DecodeError { format: String, message: String },
    IoError { path: String, message: String },
    ConfigError { message: String },
    UnknownError { message: String },
}

impl Display for DocusError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DocusError::RenderError { path, message } => {
                write!(f, "RenderError: {} {}", path, message)
            }
            DocusError::EncodeError { format, message } => {
                write!(f, "EncodeError: {} {}", format, message)
            }
            DocusError::DecodeError { format, message } => {
                write!(f, "DecodeError: {} {}", format, message)
            }
            DocusError::IoError { path, message } => {
                write!(f, "IoError: {} at {}", message, path)
            }
            DocusError::ConfigError { message } => {
                write!(f, "ConfigError: {}", message)
            }
            DocusError::UnknownError { message } => {
                write!(f, "UnknownError: {}", message)
            }
        }
    }
}
