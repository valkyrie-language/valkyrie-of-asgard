use super::*;
use docus_core::render::build_site;

#[derive(Debug, Args)]
pub struct BuildCommand {
    #[arg(short, long, default_value = "./docs")]
    input: String,

    #[arg(short, long, default_value = "./dist")]
    output: String,

    #[arg(long)]
    cache_dir: String,
}

impl BuildCommand {
    pub async fn run(&self) -> Result<(), DocusError> {
        let cache_path = std::path::Path::new(&self.cache_dir);
        if !cache_path.exists() {
            std::fs::create_dir_all(cache_path)?;
        }
        build_site(&self.input, cache_path)?;
        Ok(())
    }
}
