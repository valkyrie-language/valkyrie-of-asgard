use crate::{DocusConfig, DocusError};
use std::path::Path;

pub fn build_site(config: &DocusConfig) -> Result<(), DocusError> {
    // 创建多语言输出目录
    for lang in config.i18n.languages.keys() {
        let lang_path = crate::i18n::build_lang_path(&config.output_dir, lang);
        std::fs::create_dir_all(&lang_path)?;
    }

    // 按语言回退链复制文档文件
    for (lang, config) in &config.i18n.languages {
        let fallback_chain = config.i18n.resolve_fallback_chain(lang);
        let lang_output_dir = crate::i18n::build_lang_path(&config.output_dir, lang);

        // 遍历源文件目录
        for entry in walkdir::WalkDir::new("docs") {
            let entry = entry.map_err(|e| DocusError::IoError(e.to_string()))?;
            if entry.file_type().is_file() {
                let path = entry.path();
                let rel_path = path.strip_prefix("docs").map_err(|_| DocusError::PathError)?;

                // 查找优先级：当前语言 -> 回退语言 -> 主语言
                let mut source_file = None;
                for check_lang in &fallback_chain {
                    let lang_path = Path::new("docs")
                        .join(check_lang)
                        .join(rel_path);
                    
                    if lang_path.exists() {
                        source_file = Some(lang_path);
                        break;
                    }
                }

                // 复制文件到对应语言目录
                if let Some(src) = source_file {
                    let dest = lang_output_dir.join(rel_path);
                    std::fs::create_dir_all(dest.parent().unwrap())?;
                    std::fs::copy(&src, &dest)?;
                }
            }
        }
    }

    Ok(())
}