mod handlers;
mod models;
mod vector_store;

use anyhow::Result;
use clap::Parser;
use std::sync::Arc;
use tokio::sync::Mutex;

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
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    tracing::info!("Starting Review Search Backend on port {}", args.port);
    tracing::info!("Data directory: {}", args.data_dir);

    // Initialize vector store
    let vector_store = VectorStore::new(&args.data_dir).await?;
    let app_state: AppState = Arc::new(Mutex::new(vector_store));

    // Create router
    let app = create_router(app_state);

    // Start server
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", args.port)).await?;
    
    tracing::info!("Server running on http://0.0.0.0:{}", args.port);
    
    axum::serve(listener, app).await?;

    Ok(())
}
