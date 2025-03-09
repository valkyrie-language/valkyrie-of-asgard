use serde::{Deserialize, Deserializer, Serialize};

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct TopbarConfig {
    #[serde(default)]
    enable: bool,
}
