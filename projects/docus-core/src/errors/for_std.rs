use crate::DocusError;

impl From<std::io::Error> for DocusError {
    fn from(err: std::io::Error) -> Self {
        DocusError::IoError { path: "".to_string(), message: err.to_string() }
    }
}
