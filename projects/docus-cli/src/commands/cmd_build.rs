use super::*;

#[derive(Debug, Args)]
pub struct BuildCommand {
    #[arg(short, long, default_value = "./docs")]
    input: String,

    #[arg(short, long, default_value = "./dist")]
    output: String,
}

impl BuildCommand {
    pub async fn run(&self) -> Result<(), DocusError> {
        println!("Building from {} to {}", self.input, self.output);
        todo!()
    }
}
