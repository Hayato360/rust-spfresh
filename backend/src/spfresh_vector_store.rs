use crate::spfresh_bindings::SPFreshIndexWrapper;
use crate::models::{Review, InsertReviewRequest};
use crate::fastembed_service::FastEmbedService;
use anyhow::Result;
use serde_json;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use uuid::Uuid;

pub struct SPFreshVectorStore {
    index: Arc<RwLock<Option<SPFreshIndexWrapper>>>,
    reviews: Arc<RwLock<Vec<Review>>>,
    id_to_review_index: Arc<RwLock<HashMap<i32, usize>>>,
    fastembed_service: FastEmbedService,
    data_dir: String,
    dimension: usize,
    next_id: Arc<RwLock<i32>>,
}

impl SPFreshVectorStore {
    pub async fn new(data_dir: &str, fastembed_service: FastEmbedService) -> Result<Self> {
        // Create SPFresh index with FastEmbed dimension (384)
        let dimension = 384;
        let index = match SPFreshIndexWrapper::new(dimension) {
            Ok(idx) => Some(idx),
            Err(e) => {
                eprintln!("Failed to create SPFresh index: {}", e);
                None
            }
        };

        let store = Self {
            index: Arc::new(RwLock::new(index)),
            reviews: Arc::new(RwLock::new(Vec::new())),
            id_to_review_index: Arc::new(RwLock::new(HashMap::new())),
            fastembed_service,
            data_dir: data_dir.to_string(),
            dimension,
            next_id: Arc::new(RwLock::new(0)),
        };

        // Load existing data if any
        store.load_existing_data().await?;
        
        Ok(store)
    }

    async fn load_existing_data(&self) -> Result<()> {
        let metadata_path = format!("{}/spfresh_reviews.jsonl", self.data_dir);
        
        if !Path::new(&metadata_path).exists() {
            return Ok(());
        }

        let file = File::open(&metadata_path)?;
        let reader = BufReader::new(file);

        let mut reviews = self.reviews.write().await;
        let mut id_to_review_index = self.id_to_review_index.write().await;
        let mut next_id = self.next_id.write().await;

        for line in reader.lines() {
            let line = line?;
            if !line.trim().is_empty() {
                let review: Review = serde_json::from_str(&line)?;
                
                // Generate embedding for the review
                let text_to_embed = format!("{} {}", review.review_title, review.review_body);
                let embedding = self.fastembed_service.embed_text(&text_to_embed)?;

                // Add to SPFresh index
                if let Some(ref index) = *self.index.read().await {
                    let vector_id = match index.add_vector(&embedding, Some(&review.id)) {
                        Ok(id) => id,
                        Err(e) => {
                            eprintln!("Failed to add vector to SPFresh index: {}", e);
                            continue;
                        }
                    };

                    let review_index = reviews.len();
                    id_to_review_index.insert(vector_id, review_index);
                    reviews.push(review);
                    
                    if vector_id >= *next_id {
                        *next_id = vector_id + 1;
                    }
                }
            }
        }

        println!("Loaded {} existing reviews into SPFresh index", reviews.len());
        Ok(())
    }

    pub async fn insert_review(&self, request: InsertReviewRequest) -> Result<Review> {
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
        let embedding = self.fastembed_service.embed_text(&text_to_embed)?;

        // Add to SPFresh index
        let vector_id = if let Some(ref index) = *self.index.read().await {
            match index.add_vector(&embedding, Some(&review.id)) {
                Ok(id) => id,
                Err(e) => {
                    return Err(anyhow::anyhow!("Failed to add vector to SPFresh index: {}", e));
                }
            }
        } else {
            return Err(anyhow::anyhow!("SPFresh index not initialized"));
        };

        // Add to in-memory storage
        let mut reviews = self.reviews.write().await;
        let review_index = reviews.len();
        reviews.push(review.clone());

        let mut id_to_review_index = self.id_to_review_index.write().await;
        id_to_review_index.insert(vector_id, review_index);

        // Update next ID
        let mut next_id = self.next_id.write().await;
        if vector_id >= *next_id {
            *next_id = vector_id + 1;
        }

        // Append to file storage
        self.append_to_storage(&review).await?;

        Ok(review)
    }

    async fn append_to_storage(&self, review: &Review) -> Result<()> {
        // Ensure data directory exists
        std::fs::create_dir_all(&self.data_dir)?;

        // Append to metadata file
        let metadata_path = format!("{}/spfresh_reviews.jsonl", self.data_dir);
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(metadata_path)?;
        
        let json_line = serde_json::to_string(review)?;
        writeln!(file, "{}", json_line)?;

        Ok(())
    }

    pub async fn search(&self, query: &str, limit: usize) -> Result<Vec<(Review, f32)>> {
        if let Some(ref index) = *self.index.read().await {
            // Generate FastEmbed embedding for query
            let query_vector = self.fastembed_service.embed_text(query)?;

            // Search using SPFresh
            let search_results = match index.search(&query_vector, limit) {
                Ok(results) => results,
                Err(e) => {
                    eprintln!("Search failed: {}", e);
                    return Ok(Vec::new());
                }
            };

            // Convert results to reviews
            let reviews = self.reviews.read().await;
            let id_to_review_index = self.id_to_review_index.read().await;
            
            let mut results = Vec::new();
            for (vector_id, distance) in search_results {
                if let Some(&review_index) = id_to_review_index.get(&vector_id) {
                    if let Some(review) = reviews.get(review_index) {
                        // Convert distance to similarity (assuming distance is based on cosine or similar)
                        let similarity = 1.0 - distance;
                        results.push((review.clone(), similarity));
                    }
                }
            }

            Ok(results)
        } else {
            Ok(Vec::new())
        }
    }

    pub async fn get_stats(&self) -> (usize, usize) {
        let reviews = self.reviews.read().await;
        let vector_count = if let Some(ref index) = *self.index.read().await {
            index.get_vector_count()
        } else {
            0
        };
        
        (reviews.len(), vector_count)
    }

    pub async fn is_ready(&self) -> bool {
        let index = self.index.read().await;
        index.is_some()
    }
}
