#[derive(Debug, Clone)]
pub enum DocusError {
    RenderError { path: String, message: String },
    EncodeError { format: String, message: String },
    DecodeError { format: String, message: String },
}

pub type Result<T> = std::result::Result<T, DocusError>;
