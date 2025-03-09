use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StyleConfig {
    #[serde(default)]
    pub theme: String,
    #[serde(default)]
    pub variables: std::collections::HashMap<String, String>,
}

impl Default for StyleConfig {
    fn default() -> Self {
        Self { theme: "light".to_string(), variables: Default::default() }
    }
}
