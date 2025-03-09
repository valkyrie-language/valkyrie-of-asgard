use crate::DocusError;
use serde::{
    de::{MapAccess, Visitor}, Deserialize, Deserializer,
    Serialize,
};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Serialize)]
pub struct InternationalizationConfig {
    #[serde(rename = "main")]
    pub default_lang: String,
    pub languages: BTreeMap<String, LanguageConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    #[serde(skip)]
    pub fallback_chain: Vec<String>,
}

impl<'de> Deserialize<'de> for InternationalizationConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let visitor = InternationalizationVisitor { default_lang: "en-us".to_string() };
        deserializer.deserialize_map(visitor)
    }
}

pub struct InternationalizationVisitor {
    pub default_lang: String,
}

impl<'de> Visitor<'de> for InternationalizationVisitor {
    type Value = InternationalizationConfig;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map")
    }

    fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut result =
            InternationalizationConfig { default_lang: "en-us".to_ascii_lowercase(), languages: BTreeMap::default() };
        result.insert(LanguageConfig {
            code: "en-us".to_ascii_lowercase(),
            icon: None,
            name: Some("English".to_string()),
            fallback: None,
            fallback_chain: vec![],
        });
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "main" => self.default_lang = map.next_value()?,
                "languages" => result.insert(map.next_value()?),
                _ => {}
            }
        }
        result.resolve_fallback_chain();
        Ok(result)
    }
}

impl InternationalizationConfig {
    /// find feedback file path
    ///
    /// /<PATH>/<FEEDBACK_NAME>.<LANGUAGE>.html
    pub fn find_feedback_file(&self, file: &Path, target_lang: &str) -> Result<PathBuf, DocusError> {
        let name = file.file_name().unwrap().to_str().unwrap();
        let ext = file.extension().unwrap().to_str().unwrap();
        let config = self.languages.get(target_lang).unwrap();
        for lang in config.fallback_chain.as_slice() {
            let path = file.parent().unwrap().join(format!("{}.{}.{}", name, lang, ext));
            if path.exists() {
                return Ok(path);
            }
        }
        Err(DocusError::IoError {
            path: file.display().to_string(),
            message: format!("feedback file not found: {}", file.display()),
        })
    }

    /// find fallback chain of the languages
    ///
    /// error when there is a ring or no fallback language (except main language)
    fn resolve_fallback_chain(&mut self) -> Result<(), DocusError> {
        todo!()
    }

    fn insert(&mut self, value: LanguageConfig) {
        self.languages.insert(value.code.to_string(), value);
    }
}
