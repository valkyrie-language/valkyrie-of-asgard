use crate::DocusError;
use notify::Error;

impl From<Error> for DocusError {
    fn from(e: Error) -> Self {
        crate::errors::DocusError::RenderError { path: "".to_string(), message: e.to_string() }
    }
}
