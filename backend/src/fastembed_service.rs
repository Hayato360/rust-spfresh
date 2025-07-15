use anyhow::{Context, Result};
use std::sync::{Arc, Mutex};
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};

pub struct FastEmbedService {
    model: Arc<Mutex<TextEmbedding>>,
}

impl FastEmbedService {
    pub fn new() -> Result<Self> {
        tracing::info!("Initializing FastEmbed service...");
        
        let model = TextEmbedding::try_new(
            InitOptions::new(EmbeddingModel::BGESmallENV15).with_show_download_progress(true),
        )
        .context("Failed to initialize FastEmbed model")?;

        tracing::info!("FastEmbed model initialized successfully");

        Ok(Self {
            model: Arc::new(Mutex::new(model)),
        })
    }

    pub fn embed_text(&self, text: &str) -> Result<Vec<f32>> {
        let mut model = self.model.lock().map_err(|e| anyhow::anyhow!("Failed to lock model: {}", e))?;
        
        let embeddings = model
            .embed(vec![text], None)
            .context("Failed to generate embeddings")?;

        embeddings
            .into_iter()
            .next()
            .context("No embedding generated")
    }

    pub fn embed_batch(&self, texts: Vec<&str>) -> Result<Vec<Vec<f32>>> {
        let mut model = self.model.lock().map_err(|e| anyhow::anyhow!("Failed to lock model: {}", e))?;
        
        let embeddings = model
            .embed(texts, None)
            .context("Failed to generate batch embeddings")?;

        Ok(embeddings)
    }

    pub fn get_embedding_dimension() -> usize {
        384 // BGE-small-en-v1.5 produces 384-dimensional embeddings
    }
}

impl Clone for FastEmbedService {
    fn clone(&self) -> Self {
        Self {
            model: Arc::clone(&self.model),
        }
    }
}
