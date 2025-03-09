use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct InternationalizationConfig {
    pub main: String,
    pub languages: HashMap<String, LanguageConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LanguageConfig {
    /// language code
    pub code: String,
    /// icon path in `/assets`
    pub icon: Option<String>,
    /// language display name
    ///
    /// fallback with language code
    pub name: Option<String>,
    /// fallback language code
    pub fallback: Option<String>,
}

impl InternationalizationConfig {
    pub fn resolve_fallback_chain(&self, lang: &str) -> Vec<&str> {
        let mut chain = vec![lang];
        let mut current = lang;

        while let Some(fallback) = self.languages.get(current).and_then(|c| c.fallback.as_deref()) {
            if chain.contains(&fallback) {
                break;
            }
            chain.push(fallback);
            current = fallback;
        }

        chain
    }

    pub fn validate(&self) -> Result<(), String> {
        // 验证fallback链是否有效
        for (code, config) in &self.languages {
            if let Some(fallback) = &config.fallback {
                if !self.languages.contains_key(fallback) {
                    return Err(format!("Invalid fallback language '{}' for '{}'", fallback, code));
                }
            }
        }
        Ok(())
    }
}

pub fn build_lang_path(output_dir: &str, lang: &str) -> String {
    format!("{}/{}", output_dir, lang.replace('-', "_"))
}
