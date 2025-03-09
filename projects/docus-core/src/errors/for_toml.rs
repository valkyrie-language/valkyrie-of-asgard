use crate::DocusError;
use toml::de::Error;

impl From<Error> for DocusError {
    fn from(e: Error) -> Self {
        DocusError::DecodeError { format: "toml".to_string(), message: e.to_string() }
    }
}
