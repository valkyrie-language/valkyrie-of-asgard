use docus_core::DocusError;
use std::path::PathBuf;

pub fn find_or_create_cache_dir(default: &Option<String>) -> Result<PathBuf, DocusError> {
    let cache_dir = match default {
        Some(s) => {
            let path = PathBuf::from(s)
                .canonicalize()
                .map_err(|e| DocusError::IoError(format!("无效缓存路径: {}", e)))?;
            
            if !path.is_dir() {
                std::fs::create_dir_all(&path)
                    .map_err(|e| DocusError::IoError(format!("创建缓存目录失败: {}", e)))?;
            }
            path
        }
        None => {
            let current_dir = std::env::current_dir()
                .map_err(|e| DocusError::IoError(format!("获取当前目录失败: {}", e)))?;
            
            current_dir.ancestors()
                .find_map(|p| {
                    let target = p.join("target");
                    target.is_dir().then(|| target.join("docus"))
                })
                .unwrap_or_else(|| current_dir.join("target/docus"))
        }
    };

    // 检查目录可写性
    let test_file = cache_dir.join(".write_test");
    std::fs::write(&test_file, "")
        .and_then(|_| std::fs::remove_file(&test_file))
        .map_err(|e| DocusError::IoError(format!("目录不可写: {} - {}", cache_dir.display(), e)))?;

    Ok(cache_dir)
}
