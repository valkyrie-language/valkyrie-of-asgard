use crate::DocusError;
use std::panic::Location;

impl From<std::io::Error> for DocusError {
    #[track_caller]
    fn from(err: std::io::Error) -> Self {
        let location = Location::caller();

        DocusError::IoError { path: location.to_string(), message: err.to_string() }
    }
}
