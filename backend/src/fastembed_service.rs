use anyhow::Result;

#[cfg(unix)]
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};

pub struct FastEmbedService {
    #[cfg(unix)]
    model: Arc<Mutex<TextEmbedding>>,
    #[cfg(windows)]
    _placeholder: (),
}

impl FastEmbedService {
    pub fn new() -> Result<Self> {
        tracing::info!("Initializing FastEmbed service...");
        
        #[cfg(unix)]
        {
            let model = TextEmbedding::try_new(
                InitOptions::new(EmbeddingModel::BGESmallENV15).with_show_download_progress(true),
            )
            .context("Failed to initialize FastEmbed model")?;

            tracing::info!("FastEmbed model initialized successfully");

            Ok(Self {
                model: Arc::new(Mutex::new(model)),
            })
        }
        
        #[cfg(windows)]
        {
            tracing::warn!("FastEmbed not available on Windows, using fallback embedding");
            Ok(Self {
                _placeholder: (),
            })
        }
    }

    pub fn embed_text(&self, text: &str) -> Result<Vec<f32>> {
        #[cfg(unix)]
        {
            let mut model = self.model.lock().map_err(|e| anyhow::anyhow!("Failed to lock model: {}", e))?;
            
            let embeddings = model
                .embed(vec![text], None)
                .context("Failed to generate embeddings")?;

            embeddings
                .into_iter()
                .next()
                .context("No embedding generated")
        }
        
        #[cfg(windows)]
        {
            // Fallback to simple hash-based embedding for Windows
            self.create_simple_embedding(text)
        }
    }

    pub fn embed_batch(&self, texts: Vec<&str>) -> Result<Vec<Vec<f32>>> {
        #[cfg(unix)]
        {
            let mut model = self.model.lock().map_err(|e| anyhow::anyhow!("Failed to lock model: {}", e))?;
            
            let embeddings = model
                .embed(texts, None)
                .context("Failed to generate batch embeddings")?;

            Ok(embeddings)
        }
        
        #[cfg(windows)]
        {
            let mut results = Vec::new();
            for text in texts {
                results.push(self.create_simple_embedding(text)?);
            }
            Ok(results)
        }
    }

    pub fn get_embedding_dimension() -> usize {
        #[cfg(unix)]
        {
            384 // BGE-small-en-v1.5 produces 384-dimensional embeddings
        }
        
        #[cfg(windows)]
        {
            128 // Fallback embedding dimension
        }
    }

    #[cfg(windows)]
    fn create_simple_embedding(&self, text: &str) -> Result<Vec<f32>> {
        // Simple hash-based embedding for Windows fallback
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let lowercase_text = text.to_lowercase();
        let words: Vec<&str> = lowercase_text.split_whitespace().collect();
        let mut embedding = vec![0.0; Self::get_embedding_dimension()];
        
        for word in &words {
            let mut hasher = DefaultHasher::new();
            word.hash(&mut hasher);
            let hash = hasher.finish();
            
            for i in 0..embedding.len() {
                let idx = (hash as usize + i) % embedding.len();
                embedding[idx] += 1.0;
            }
        }
        
        // Normalize
        let norm = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            embedding.iter_mut().for_each(|x| *x /= norm);
        }
        
        Ok(embedding)
    }
}

impl Clone for FastEmbedService {
    fn clone(&self) -> Self {
        #[cfg(unix)]
        {
            Self {
                model: Arc::clone(&self.model),
            }
        }
        
        #[cfg(windows)]
        {
            Self {
                _placeholder: (),
            }
        }
    }
}
