#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

use clap::{Parser, Subcommand};
use docus_core::DocusError;

#[derive(Parser)]
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

#[derive(Subcommand)]
pub enum Commands {
    /// 构建静态站点
    Build {
        #[arg(short, long, default_value = "./docs")]
        input: String,

        #[arg(short, long, default_value = "./dist")]
        output: String,
    },

    /// 启动开发服务器
    Serve {
        #[arg(short, long, default_value = "127.0.0.1")]
        host: String,

        #[arg(short, long, default_value_t = 3000)]
        port: u16,

        #[arg(long, default_value = "./docs")]
        watch: String,
    },
}

impl DocusCLI {
    pub async fn run(&self) -> Result<(), DocusError> {
        match &self.command {
            Commands::Build { input, output } => {
                println!("Building from {} to {}", input, output);
                // 调用构建逻辑
            }
            Commands::Serve { host, port, watch } => {
                println!("Serving on {}:{}, watching {}", host, port, watch);
                // 调用服务启动逻辑
            }
        }
    }
}
