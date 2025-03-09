use docus_core::DocusError;

pub fn find_or_create_cache_dir() -> Result<String, DocusError> {
    let current_dir = std::env::current_dir()?;
    for ancestor in current_dir.ancestors() {
        let target = ancestor.join("target");
        if target.is_file() {
            let cache_dir = target.parent().unwrap().join("target/docus");
            return Ok(cache_dir.to_str().unwrap().to_string());
        }
    }
    let fallback_dir = current_dir.join("target/docus");
    std::fs::create_dir_all(&fallback_dir)?;
    Ok(fallback_dir.to_str().unwrap().to_string())
}
