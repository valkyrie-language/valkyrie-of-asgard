use std::path::PathBuf;
use std::sync::Arc;
use axum::{
    extract::State,
    response::Html,
    routing::get,
    Router,
};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::broadcast;
use tracing::info;

use docus_core::config::{DocusConfig, SidebarConfig, TopbarConfig};
use docus_core::markdown::MarkdownRenderer;

#[derive(Clone)]
pub struct ServerState {
    config: Arc<DocusConfig>,
    sidebar: Arc<SidebarConfig>,
    topbar: Arc<TopbarConfig>,
    renderer: Arc<MarkdownRenderer>,
}

pub struct DevServer {
    state: ServerState,
    watcher: Option<RecommendedWatcher>,
    reload_tx: broadcast::Sender<()>,
}

impl DevServer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = DocusConfig::load()?;
        let sidebar = SidebarConfig::load()?;
        let topbar = TopbarConfig::load()?;
        let renderer = MarkdownRenderer::new();

        let state = ServerState {
            config: Arc::new(config),
            sidebar: Arc::new(sidebar),
            topbar: Arc::new(topbar),
            renderer: Arc::new(renderer),
        };

        let (reload_tx, _) = broadcast::channel(100);

        Ok(Self {
            state,
            watcher: None,
            reload_tx,
        })
    }

    pub async fn serve(&mut self, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        let app = Router::new()
            .route("/", get(handle_root))
            .route("/*path", get(handle_page))
            .with_state(self.state.clone());

        let addr = format!("127.0.0.1:{}", port);
        info!("Server starting on http://{}", addr);

        self.setup_watcher()?;

        axum::Server::bind(&addr.parse()?)
            .serve(app.into_make_service())
            .await?
;
        Ok(())
    }

    fn setup_watcher(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let reload_tx = self.reload_tx.clone();
        let mut watcher = notify::recommended_watcher(move |res| {
            if let Ok(_) = res {
                let _ = reload_tx.send(());
            }
        })?;

        watcher.watch(PathBuf::from("docs").as_path(), RecursiveMode::Recursive)?;
        self.watcher = Some(watcher);
        Ok(())
    }
}

async fn handle_root(State(state): State<ServerState>) -> Html<String> {
    let content = std::fs::read_to_string("docs/index.md").unwrap_or_else(|_| "# Welcome\n\nNo index page found.".to_string());
    let html = state.renderer.render_string(&content);
    Html(html)
}

async fn handle_page(State(state): State<ServerState>) -> Html<String> {
    // TODO: Implement page handling with proper routing and template rendering
    Html(String::from("<h1>Page</h1>"))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let mut server = DevServer::new()?;
    server.serve(3000).await
}