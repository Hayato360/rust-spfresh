use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub id: String,
    pub review_title: String,
    pub review_body: String,
    pub product_id: String,
    pub review_rating: u8,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsertReviewRequest {
    pub review_title: String,
    pub review_body: String,
    pub product_id: String,
    pub review_rating: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchRequest {
    pub query: String,
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub reviews: Vec<Review>,
    pub total_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewWithScore {
    pub review: Review,
    pub similarity_score: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }
    
    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}
