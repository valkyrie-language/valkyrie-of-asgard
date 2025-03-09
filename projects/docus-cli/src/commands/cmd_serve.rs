use super::*;

#[derive(Debug, Args)]
pub struct ServeCommand {
    #[arg(short, long, default_value = "127.0.0.1")]
    host: String,

    #[arg(short, long, default_value_t = 3000)]
    port: u16,

    #[arg(long, default_value = "./docs")]
    watch: String,
}

impl ServeCommand {
    pub async fn run(&self) -> Result<(), DocusError> {
        todo!()
    }
}
