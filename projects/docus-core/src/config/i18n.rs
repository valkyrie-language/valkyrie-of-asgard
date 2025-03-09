use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct I18nConfig {
    pub main_lang: String,
    pub languages: HashMap<String, LanguageConfig>,
}

#[derive(Debug, Clone)]
pub struct LanguageConfig {
    pub code: String,
    pub fallback: Option<String>,
}

impl I18nConfig {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fallback_chain() {
        let mut config = I18nConfig {
            main_lang: "en-us".to_string(),
            languages: HashMap::new(),
        };

        config.languages.insert("zh-hans".to_string(), LanguageConfig {
            code: "zh-hans".to_string(),
            fallback: Some("en-us".to_string()),
        });

        config.languages.insert("en-us".to_string(), LanguageConfig {
            code: "en-us".to_string(),
            fallback: None,
        });

        assert_eq!(config.resolve_fallback_chain("zh-hans"), vec!["zh-hans", "en-us"]);
    }
}