use super::*;
use crate::helpers::find_or_create_cache_dir;
use docus_core::render::build_site;

#[derive(Debug, Args)]
pub struct BuildCommand {
    #[arg(short, long, default_value = "./docs")]
    input: String,

    #[arg(short, long, default_value = "./dist")]
    output: String,

    #[arg(long)]
    cache: Option<String>,
}

impl BuildCommand {
    pub async fn run(&self) -> Result<(), DocusError> {
        let cache_path = find_or_create_cache_dir(&self.cache)?;
        build_site(&self.input, cache_path)?;
        Ok(())
    }
}
