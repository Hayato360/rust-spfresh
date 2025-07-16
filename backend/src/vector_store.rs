use anyhow::{Context, Result};
use serde_json;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write, Read};
use std::path::Path;
use uuid::Uuid;

use crate::fastembed_service::FastEmbedService;
use crate::models::{Review, InsertReviewRequest};

pub struct VectorStore {
    data_dir: String,
    vectors: Vec<Vec<f32>>,
    reviews: Vec<Review>,
    fastembed_service: FastEmbedService,
}

impl VectorStore {
    pub async fn new(data_dir: &str, fastembed_service: FastEmbedService) -> Result<Self> {
        let mut store = Self {
            data_dir: data_dir.to_string(),
            vectors: Vec::new(),
            reviews: Vec::new(),
            fastembed_service,
        };

        // Load existing data if any
        store.load_existing_data().await?;
        
        Ok(store)
    }

    async fn load_existing_data(&mut self) -> Result<()> {
        let metadata_path = format!("{}/reviews.jsonl", self.data_dir);
        let vector_path = format!("{}/reviews.index", self.data_dir);
        
        if !Path::new(&metadata_path).exists() {
            return Ok(());
        }

        // Load vectors from binary file if exists .
        if Path::new(&vector_path).exists() {
            self.load_vectors_from_file(&vector_path).await?;
        }

        // Load metadata from JSONL
        let file = File::open(&metadata_path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            if !line.trim().is_empty() {
                let review: Review = serde_json::from_str(&line)
                    .context("Failed to deserialize review from JSONL")?;
                
                self.reviews.push(review);
            }
        }

        // If vector count doesn't match review count, regenerate vectors
        if self.vectors.len() != self.reviews.len() {
            tracing::warn!("Vector count ({}) doesn't match review count ({}), regenerating vectors from text using FastEmbed", 
                          self.vectors.len(), self.reviews.len());
            
            // Clear existing vectors
            self.vectors.clear();
            
            // Generate vectors for all reviews
            for review in &self.reviews {
                let text_to_embed = format!("{} {}", review.review_title, review.review_body);
                let embedding = self.create_fastembed_embedding(&text_to_embed)?;
                self.vectors.push(embedding);
            }
            
            // Save the regenerated vectors (overwrite the file)
            self.save_all_vectors_to_file().await?;
        }

        tracing::info!("Loaded {} existing reviews with {} vectors", self.reviews.len(), self.vectors.len());
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

        // Generate FastEmbed embedding
        let text_to_embed = format!("{} {}", request.review_title, request.review_body);
        let embedding = self.create_fastembed_embedding(&text_to_embed)?;

        // Append to in-memory storage
        self.vectors.push(embedding.clone());
        self.reviews.push(review.clone());

        // Append to file storage (both metadata and vector)
        self.append_to_storage(&review, &embedding).await?;

        Ok(review)
    }

    async fn append_to_storage(&self, review: &Review, embedding: &Vec<f32>) -> Result<()> {
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

        // Append vector to binary index file
        self.append_vector_to_file(embedding).await?;

        Ok(())
    }

    pub async fn search(&self, query: &str, limit: usize) -> Result<Vec<(Review, f32)>> {
        if self.vectors.is_empty() {
            return Ok(Vec::new());
        }

        // Generate FastEmbed embedding for query
        let query_vector = self.create_fastembed_embedding(query)?;

        // Calculate cosine similarities
        let mut similarities: Vec<(usize, f32)> = self.vectors
            .iter()
            .enumerate()
            .map(|(idx, vector)| {
                let similarity = cosine_similarity(&query_vector, vector);
                tracing::debug!("Similarity for review {}: {:.4}", idx, similarity);
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

    // FastEmbed-based embedding generation
    fn create_fastembed_embedding(&self, text: &str) -> Result<Vec<f32>> {
        self.fastembed_service.embed_text(text)
    }

    // Legacy simple TF-IDF style embedding (kept for backward compatibility)
    // In production, you'd use a proper embedding model
    #[allow(dead_code)]
    fn create_simple_embedding(&self, text: &str) -> Vec<f32> {
        let lowercase_text = text.to_lowercase();
        let words: Vec<&str> = lowercase_text
            .split_whitespace()
            .collect();

        // Simplified vocabulary with only positive and negative categories
        let positive_words = vec![
            "good", "great", "excellent", "amazing", "wonderful", "fantastic", 
            "love", "perfect", "best", "awesome", "outstanding", "superb", 
            "brilliant", "magnificent", "exceptional", "marvelous", "incredible", 
            "remarkable", "impressive", "stunning", "fabulous", "terrific",
            "gorgeous", "beautiful", "lovely", "nice", "pleasant", "enjoyable",
            "delightful", "charming", "attractive", "appealing", "desirable",
            "superior", "premium", "top", "first-class", "high-quality", "fine",
            "phenomenal", "friendly", "helpful", "patient", "knowledgeable",
            "attentive", "cool", "affordable", "comfortable", "easy", "fast",
            "durable", "portable", "compact", "smooth", "soft", "responsive",
            "intuitive", "simple", "modern", "sleek", "elegant", "recommend",
            "satisfied", "happy", "pleased", "worth", "enjoy", "appreciate",
            "praise", "reliable", "stable", "clear", "bright", "quiet",
            "quality", "value", "recommend", "buy", "purchase"
        ];
        
        let negative_words = vec![
            "bad", "poor", "terrible", "awful", "hate", "worst", 
            "horrible", "disappointing", "useless", "pathetic", "disgusting",
            "dreadful", "appalling", "shocking", "disastrous", "catastrophic",
            "horrendous", "atrocious", "abysmal", "deplorable", "miserable",
            "inferior", "substandard", "defective", "faulty", "broken",
            "worthless", "garbage", "trash", "junk", "rubbish", "cheap",
            "flimsy", "fragile", "unreliable", "unstable", "problematic",
            "suspicious", "difficult", "slow", "expensive", "heavy", "rough",
            "hard", "laggy", "confusing", "complex", "outdated", "bulky",
            "disappointed", "regret", "criticize", "complain", "blame",
            "blurry", "dark", "loud", "uncomfortable", "return", "refund"
        ];

        // Create vocabulary with only positive and negative words
        let mut vocabulary: Vec<&str> = Vec::new();
        vocabulary.extend(&positive_words);
        vocabulary.extend(&negative_words);

        let mut embedding = vec![0.0; vocabulary.len()];
        
        for (i, vocab_word) in vocabulary.iter().enumerate() {
            let count = words.iter().filter(|&&word| word == *vocab_word).count() as f32;
            // Simple TF with some IDF-like weighting
            embedding[i] = count * (1.0 + (vocabulary.len() as f32 / (count + 1.0)).ln());
        }

        // Add sentiment-focused features
        let positive_count = words.iter().filter(|&&word| positive_words.contains(&word)).count() as f32;
        let negative_count = words.iter().filter(|&&word| negative_words.contains(&word)).count() as f32;
        let sentiment_score = if positive_count + negative_count > 0.0 {
            (positive_count - negative_count) / (positive_count + negative_count)
        } else {
            0.0
        };
        
        let mut features = vec![
            words.len() as f32 / 10.0,  // Text length feature
            positive_count,             // Positive word count
            negative_count,             // Negative word count
            sentiment_score,            // Sentiment balance (-1 to 1)
        ];

        embedding.append(&mut features);

        // Normalize the vector
        let norm = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            embedding.iter_mut().for_each(|x| *x /= norm);
        }

        embedding
    }

    // Vector file management functions
    async fn append_vector_to_file(&self, embedding: &Vec<f32>) -> Result<()> {
        let vector_path = format!("{}/reviews.index", self.data_dir);
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(vector_path)?;

        // Write vector dimension first (as u32)
        let dim = embedding.len() as u32;
        file.write_all(&dim.to_le_bytes())?;
        
        // Write vector components
        for &value in embedding {
            file.write_all(&value.to_le_bytes())?;
        }
        
        file.flush()?;
        Ok(())
    }

    async fn load_vectors_from_file(&mut self, vector_path: &str) -> Result<()> {
        let mut file = File::open(vector_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        
        let mut cursor = 0;
        
        while cursor + 4 <= buffer.len() {
            // Read dimension
            let dim_bytes = &buffer[cursor..cursor + 4];
            let dim = u32::from_le_bytes([dim_bytes[0], dim_bytes[1], dim_bytes[2], dim_bytes[3]]) as usize;
            cursor += 4;
            
            // Check if we have enough bytes for the vector
            if cursor + (dim * 4) > buffer.len() {
                break;
            }
            
            // Read vector components
            let mut vector = Vec::with_capacity(dim);
            for _ in 0..dim {
                let value_bytes = &buffer[cursor..cursor + 4];
                let value = f32::from_le_bytes([value_bytes[0], value_bytes[1], value_bytes[2], value_bytes[3]]);
                vector.push(value);
                cursor += 4;
            }
            
            self.vectors.push(vector);
        }
        
        tracing::info!("Loaded {} vectors from file", self.vectors.len());
        Ok(())
    }

    async fn save_all_vectors_to_file(&self) -> Result<()> {
        let vector_path = format!("{}/reviews.index", self.data_dir);
        let mut file = File::create(vector_path)?;
        
        for vector in &self.vectors {
            // Write vector dimension first (as u32)
            let dim = vector.len() as u32;
            file.write_all(&dim.to_le_bytes())?;
            
            // Write vector components
            for &value in vector {
                file.write_all(&value.to_le_bytes())?;
            }
        }
        
        file.flush()?;
        tracing::info!("Saved {} vectors to file", self.vectors.len());
        Ok(())
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
