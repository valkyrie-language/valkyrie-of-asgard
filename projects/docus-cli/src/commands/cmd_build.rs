use super::*;
use crate::helpers::find_or_create_cache_dir;
use docus_core::render::build_site;
use std::{fs::create_dir_all, path::Path};

#[derive(Debug, Args)]
pub struct BuildCommand {
    #[arg(default_value = ".")]
    input: String,

    #[arg(default_value = "dist")]
    output: String,

    #[arg(long)]
    cache: Option<String>,
}

impl BuildCommand {
    pub async fn run(&self) -> Result<(), DocusError> {
        let input = Path::new(&self.input);
        let output_path = input.join(&self.output);
        let cache_path = find_or_create_cache_dir(&self.cache)?;
        if !input.join("docus.toml").exists() {
            let fullpath = input.canonicalize()?;
            return Err(DocusError::IoError {
                path: fullpath.display().to_string(),
                message: "`docus.toml` not found".to_string(),
            });
        }
        create_dir_all(&cache_path)?;
        create_dir_all(&output_path)?;
        build_site(input, &output_path, &cache_path)?;
        Ok(())
    }
}
