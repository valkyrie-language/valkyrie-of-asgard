#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

use crate::commands::{BuildCommand, ServeCommand};
use clap::{Parser, Subcommand};
use docus_core::DocusError;

mod commands;
mod helpers;

#[derive(Debug, Parser)]
#[command(name = "docus")]
#[command(author, version, about, long_about = None)]
pub struct DocusCLI {
    #[command(subcommand)]
    command: Commands,

    #[arg(long, global = true)]
    config: Option<std::path::PathBuf>,

    #[arg(long, global = true, default_value = "info")]
    log_level: String,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// 构建静态站点
    Build(BuildCommand),
    /// 启动开发服务器
    Serve(ServeCommand),
}

impl DocusCLI {
    pub async fn run(&self) -> Result<(), DocusError> {
        match &self.command {
            Commands::Build(cmd) => cmd.run().await,
            Commands::Serve(cmd) => cmd.run().await,
        }
    }
}
