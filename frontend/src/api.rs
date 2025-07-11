use gloo_net::http::Request;
use serde_json;

use crate::models::{ApiResponse, InsertReviewRequest, Review, SearchRequest, SearchResponse};

const BASE_URL: &str = "http://localhost:8000";

pub async fn insert_review(request: InsertReviewRequest) -> Result<Review, String> {
    let response = Request::post(&format!("{}/reviews", BASE_URL))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&request).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status() == 200 {
        let api_response: ApiResponse<Review> = response
            .json()
            .await
            .map_err(|e| e.to_string())?;
        
        if api_response.success {
            api_response.data.ok_or_else(|| "No data in successful response".to_string())
        } else {
            Err(api_response.error.unwrap_or_else(|| "Unknown error".to_string()))
        }
    } else {
        Err(format!("Server error: {}", response.status()))
    }
}

pub async fn search_reviews(query: String, limit: Option<usize>) -> Result<SearchResponse, String> {
    let request = SearchRequest { query, limit };
    
    let response = Request::post(&format!("{}/search", BASE_URL))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&request).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status() == 200 {
        let api_response: ApiResponse<SearchResponse> = response
            .json()
            .await
            .map_err(|e| e.to_string())?;
        
        if api_response.success {
            api_response.data.ok_or_else(|| "No data in successful response".to_string())
        } else {
            Err(api_response.error.unwrap_or_else(|| "Unknown error".to_string()))
        }
    } else {
        Err(format!("Server error: {}", response.status()))
    }
}

pub async fn get_stats() -> Result<(usize, usize), String> {
    let response = Request::get(&format!("{}/stats", BASE_URL))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status() == 200 {
        let api_response: ApiResponse<(usize, usize)> = response
            .json()
            .await
            .map_err(|e| e.to_string())?;
        
        if api_response.success {
            api_response.data.ok_or_else(|| "No data in successful response".to_string())
        } else {
            Err(api_response.error.unwrap_or_else(|| "Unknown error".to_string()))
        }
    } else {
        Err(format!("Server error: {}", response.status()))
    }
}
