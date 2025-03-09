use super::*;
use crate::helpers::find_or_create_cache_dir;
use docus_core::render::build_site;
use std::path::Path;

#[derive(Debug, Args)]
pub struct BuildCommand {
    #[arg(default_value = ".")]
    input: String,

    #[arg(long)]
    cache: Option<String>,
}

impl BuildCommand {
    pub async fn run(&self) -> Result<(), DocusError> {
        let dir = Path::new(&self.input);
        if !dir.join("docus.toml").exists() {
            let fullpath = dir.canonicalize()?;
            return Err(DocusError::IoError {
                path: fullpath.display().to_string(),
                message: "`docus.toml` not found".to_string(),
            });
        }
        let cache_path = find_or_create_cache_dir(&self.cache)?;
        build_site(dir, &cache_path)?;
        Ok(())
    }
}
