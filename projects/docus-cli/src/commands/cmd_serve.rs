use super::*;
use crate::helpers::find_or_create_cache_dir;
use docus_core::render::build_site;
use std::{fs::create_dir_all, path::Path};
use axum::{Router, routing::get_service, handler::HandlerWithoutStateExt};
use tower_http::services::ServeDir;
use notify::{Watcher, RecursiveMode, Event};
use tokio::sync::mpsc;

#[derive(Debug, Args)]
pub struct ServeCommand {
    #[arg(default_value = ".")]
    input: String,

    #[arg(default_value = "dist")]
    output: String,

    #[arg(long)]
    cache: Option<String>,
    
    #[arg(long, default_value = "0.0.0.0")]
    host: String,

    #[arg(long, default_value_t = 6321)]
    port: u16,
}

impl ServeCommand {
    pub async fn run(&self) -> Result<(), DocusError> {
        let input = Path::new(&self.input);
        let output_path = input.join(&self.output);
        let cache_path = find_or_create_cache_dir(&self.cache)?;

        if !input.join("docus.toml").exists() {
            let fullpath = input.canonicalize()?;
            return Err(DocusError::IoError {
                path: fullpath.display().to_string(),
                message: "`docus.toml` not found".to_string(),
            });
        }

        create_dir_all(&cache_path)?;
        create_dir_all(&output_path)?;

        // Initial build
        build_site(input, &output_path, &cache_path)?;

        // Set up file watcher
        let (tx, mut rx) = mpsc::channel(32);
        let mut watcher = notify::recommended_watcher(move |res: Result<Event, _>| {
            if let Ok(event) = res {
                let _ = tx.blocking_send(event);
            }
        })?;

        watcher.watch(input, RecursiveMode::Recursive)?;

        // Set up static file server
        let app = Router::new().nest_service("/", get_service(ServeDir::new(&output_path)));

        // Spawn the file watcher handler
        let output_path_clone = output_path.clone();
        let cache_path_clone = cache_path.clone();
        let input_clone = input.to_path_buf();

        tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                if event.kind.is_modify() || event.kind.is_create() || event.kind.is_remove() {
                    println!("Detected changes, rebuilding...");
                    if let Err(e) = build_site(&input_clone, &output_path_clone, &cache_path_clone) {
                        eprintln!("Build error: {}", e);
                    }
                }
            }
        });
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
        // Start the server
        axum::serve(listener, app.into_make_service())
            .await
            .map_err(|e| DocusError::IoError {
                path: output_path.display().to_string(),
                message: e.to_string(),
            })
    }
}
