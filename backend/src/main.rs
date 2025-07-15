mod fastembed_service;
mod handlers;
mod models;
mod vector_store;

#[cfg(test)]
mod tests;

use anyhow::Result;
use clap::Parser;
use std::sync::Arc;
use tokio::sync::Mutex;

use fastembed_service::FastEmbedService;
use handlers::{create_router, AppState};
use vector_store::VectorStore;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "8000")]
    port: u16,

    #[arg(short, long, default_value = "./data")]
    data_dir: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    tracing::info!("Starting Review Search Backend on port {}", args.port);
    tracing::info!("Data directory: {}", args.data_dir);

    // Initialize FastEmbed service
    let fastembed_service = FastEmbedService::new()?;

    // Initialize vector store with FastEmbed service
    let vector_store = VectorStore::new(&args.data_dir, fastembed_service).await?;
    let app_state: AppState = Arc::new(Mutex::new(vector_store));

    // Create router
    let app = create_router(app_state);

    // Start server
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", args.port)).await?;
    
    tracing::info!("Server running on http://0.0.0.0:{}", args.port);
    
    axum::serve(listener, app).await?;

    Ok(())
}
