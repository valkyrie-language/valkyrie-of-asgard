use clap::Parser;
use docus::DocusCLI;
use docus_core::DocusError;

#[tokio::main]
async fn main() -> Result<(), DocusError> {
    let cli = DocusCLI::parse();
    cli.run().await
}
