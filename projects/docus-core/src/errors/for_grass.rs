use crate::DocusError;
use grass::Error;

impl From<Box<Error>> for DocusError {
    fn from(e: Box<Error>) -> Self {
        DocusError::EncodeError { format: "css".to_string(), message: e.to_string() }
    }
}
