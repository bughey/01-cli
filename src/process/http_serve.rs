use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use tokio::{fs, net::TcpListener};
use tower_http::services::ServeDir;
use tracing::{info, warn};

use crate::cli::http::HttpSubCommand;

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http(subcmd: HttpSubCommand) -> Result<()> {
    match subcmd {
        HttpSubCommand::Serve(opts) => {
            process_http_serve(opts.dir, opts.port).await?;
        }
    }
    Ok(())
}

async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Direct {:?} Serve on {}", path, addr);

    let state = HttpServeState { path: path.clone() };

    let dirsrv = ServeDir::new(path);

    // axum router
    let router = axum::Router::new()
        // .route("/*path", get(file_handler))
        .nest_service("/", dirsrv)
        .with_state(Arc::new(state));

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}

#[allow(dead_code)]
async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file {:?}", p);

    if !p.exists() {
        return (
            StatusCode::NOT_FOUND,
            format!("File {} not found", p.display()),
        );
    }

    match fs::read(p).await {
        Ok(content) => {
            info!("Read {} bytes", content.len());
            (
                StatusCode::OK,
                String::from_utf8_lossy(&content).to_string(),
            )
        }
        Err(e) => {
            warn!("Error reading file: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", e))
        }
    }
}
