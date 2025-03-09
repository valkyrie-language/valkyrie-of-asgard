use super::*;
use docus_core::render::build_site;

#[derive(Debug, Args)]
pub struct BuildCommand {
    #[arg(short, long, default_value = "./docs")]
    input: String,

    #[arg(short, long, default_value = "./dist")]
    output: String,
}

impl BuildCommand {
    pub async fn run(&self) -> Result<(), DocusError> {
        build_site(&self.input)?;
        Ok(())
    }
}
