use anyhow::{Context, Result};
use serde_json;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use uuid::Uuid;

use crate::models::{Review, InsertReviewRequest};

pub struct VectorStore {
    data_dir: String,
    vectors: Vec<Vec<f32>>,
    reviews: Vec<Review>,
}

impl VectorStore {
    pub async fn new(data_dir: &str) -> Result<Self> {
        let mut store = Self {
            data_dir: data_dir.to_string(),
            vectors: Vec::new(),
            reviews: Vec::new(),
        };

        // Load existing data if any
        store.load_existing_data().await?;
        
        Ok(store)
    }

    async fn load_existing_data(&mut self) -> Result<()> {
        let metadata_path = format!("{}/reviews.jsonl", self.data_dir);
        
        if !Path::new(&metadata_path).exists() {
            return Ok(());
        }

        let file = File::open(&metadata_path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            if !line.trim().is_empty() {
                let review: Review = serde_json::from_str(&line)
                    .context("Failed to deserialize review from JSONL")?;
                
                // Generate simple TF-IDF style embedding for existing review
                let text_to_embed = format!("{} {}", review.review_title, review.review_body);
                let embedding = self.create_simple_embedding(&text_to_embed);
                
                self.vectors.push(embedding);
                self.reviews.push(review);
            }
        }

        tracing::info!("Loaded {} existing reviews", self.reviews.len());
        Ok(())
    }

    pub async fn insert_review(&mut self, request: InsertReviewRequest) -> Result<Review> {
        // Create review with unique ID and timestamp
        let review = Review {
            id: Uuid::new_v4().to_string(),
            review_title: request.review_title.clone(),
            review_body: request.review_body.clone(),
            product_id: request.product_id,
            review_rating: request.review_rating,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        // Generate simple embedding
        let text_to_embed = format!("{} {}", request.review_title, request.review_body);
        let embedding = self.create_simple_embedding(&text_to_embed);

        // Append to in-memory storage
        self.vectors.push(embedding);
        self.reviews.push(review.clone());

        // Append to file storage
        self.append_to_storage(&review).await?;

        Ok(review)
    }

    async fn append_to_storage(&self, review: &Review) -> Result<()> {
        // Ensure data directory exists
        std::fs::create_dir_all(&self.data_dir)?;

        // Append to metadata file
        let metadata_path = format!("{}/reviews.jsonl", self.data_dir);
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(metadata_path)?;
        
        let json_line = serde_json::to_string(review)?;
        writeln!(file, "{}", json_line)?;

        Ok(())
    }

    pub async fn search(&self, query: &str, limit: usize) -> Result<Vec<(Review, f32)>> {
        if self.vectors.is_empty() {
            return Ok(Vec::new());
        }

        // Generate embedding for query
        let query_vector = self.create_simple_embedding(query);

        // Calculate cosine similarities
        let mut similarities: Vec<(usize, f32)> = self.vectors
            .iter()
            .enumerate()
            .map(|(idx, vector)| {
                let similarity = cosine_similarity(&query_vector, vector);
                (idx, similarity)
            })
            .collect();

        // Sort by similarity (descending)
        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Take top results
        let results = similarities
            .into_iter()
            .take(limit)
            .map(|(idx, score)| (self.reviews[idx].clone(), score))
            .collect();

        Ok(results)
    }

    pub fn get_stats(&self) -> (usize, usize) {
        (self.reviews.len(), self.vectors.len())
    }

    // Simple TF-IDF style embedding (for demonstration)
    // In production, you'd use a proper embedding model
    fn create_simple_embedding(&self, text: &str) -> Vec<f32> {
        let lowercase_text = text.to_lowercase();
        let words: Vec<&str> = lowercase_text
            .split_whitespace()
            .collect();

        // Create a simple bag-of-words embedding with predefined vocabulary
        let vocabulary = vec![
            "good", "great", "excellent", "amazing", "wonderful", "fantastic", "love", "perfect", "best",
            "bad", "poor", "terrible", "awful", "hate", "worst", "horrible", "disappointing",
            "battery", "camera", "screen", "quality", "performance", "price", "value", "design",
            "phone", "laptop", "headphones", "device", "product", "build", "sound", "fast", "slow",
            "cheap", "expensive", "comfortable", "easy", "difficult", "heavy", "light", "durable",
            "recommend", "buy", "purchase", "money", "worth", "satisfied", "happy", "pleased"
        ];

        let mut embedding = vec![0.0; vocabulary.len()];
        
        for (i, vocab_word) in vocabulary.iter().enumerate() {
            let count = words.iter().filter(|&&word| word == *vocab_word).count() as f32;
            // Simple TF with some IDF-like weighting
            embedding[i] = count * (1.0 + (vocabulary.len() as f32 / (count + 1.0)).ln());
        }

        // Add some simple features
        let mut features = vec![
            words.len() as f32 / 10.0,  // Text length feature
            words.iter().filter(|w| w.len() > 5).count() as f32,  // Long words
            words.iter().filter(|w| w.chars().any(|c| c.is_uppercase())).count() as f32,  // Caps
        ];

        embedding.append(&mut features);

        // Normalize the vector
        let norm = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            embedding.iter_mut().for_each(|x| *x /= norm);
        }

        embedding
    }
}

// Helper function to calculate cosine similarity
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    
    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot_product / (norm_a * norm_b)
    }
}
