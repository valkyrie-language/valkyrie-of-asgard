use clap::Parser;
use docus::DocusCLI;
use docus_core::DocusError;
use tracing::level_filters::LevelFilter;

#[tokio::main]
async fn main() -> Result<(), DocusError> {
    let format = tracing_subscriber::fmt::format()
        .with_level(false) // don't include levels in formatted output
        .with_target(false) // don't include targets
        .with_thread_ids(true) // include the thread ID of the current thread
        .with_thread_names(true) // include the name of the current thread
        .compact(); // use the `Compact` formatting style.
    // Create a `fmt` subscriber that uses our custom event format, and set it
    // as the default.
    tracing_subscriber::fmt().event_format(format).with_max_level(LevelFilter::TRACE).init();
    let cli = DocusCLI::parse();
    cli.run().await
}
