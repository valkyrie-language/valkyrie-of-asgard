use clap::Args;
use docus_core::DocusError;

pub use self::{cmd_build::BuildCommand, cmd_serve::ServeCommand};
mod cmd_build;
mod cmd_serve;
