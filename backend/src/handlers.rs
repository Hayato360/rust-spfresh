use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;

use crate::models::{
    ApiResponse, InsertReviewRequest, ReviewWithScore, SearchRequest, SearchResponse,
};
use crate::vector_store::VectorStore;

pub type AppState = Arc<Mutex<VectorStore>>;

pub fn create_router(store: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/stats", get(get_stats))
        .route("/reviews", post(insert_review))
        .route("/reviews/bulk", post(insert_reviews_bulk))
        .route("/search", post(search_reviews))
        .with_state(store)
        .layer(CorsLayer::permissive())
}

async fn health_check() -> Json<ApiResponse<String>> {
    Json(ApiResponse::success("Service is healthy".to_string()))
}

async fn get_stats(State(store): State<AppState>) -> Json<ApiResponse<(usize, usize)>> {
    let store = store.lock().await;
    let stats = store.get_stats();
    Json(ApiResponse::success(stats))
}

async fn insert_review(
    State(store): State<AppState>,
    Json(request): Json<InsertReviewRequest>,
) -> Result<Json<ApiResponse<crate::models::Review>>, StatusCode> {
    let mut store = store.lock().await;
    
    match store.insert_review(request).await {
        Ok(review) => Ok(Json(ApiResponse::success(review))),
        Err(e) => {
            tracing::error!("Failed to insert review: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn insert_reviews_bulk(
    State(store): State<AppState>,
    Json(requests): Json<Vec<InsertReviewRequest>>,
) -> Result<Json<ApiResponse<Vec<crate::models::Review>>>, StatusCode> {
    let mut store = store.lock().await;
    let mut inserted_reviews = Vec::new();

    for request in requests {
        match store.insert_review(request).await {
            Ok(review) => inserted_reviews.push(review),
            Err(e) => {
                tracing::error!("Failed to insert review in bulk: {}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    }

    Ok(Json(ApiResponse::success(inserted_reviews)))
}

async fn search_reviews(
    State(store): State<AppState>,
    Json(request): Json<SearchRequest>,
) -> Result<Json<ApiResponse<SearchResponse>>, StatusCode> {
    let store = store.lock().await;
    let limit = request.limit.unwrap_or(10);

    match store.search(&request.query, limit).await {
        Ok(results) => {
            let reviews_with_scores: Vec<ReviewWithScore> = results
                .into_iter()
                .map(|(review, score)| ReviewWithScore {
                    review,
                    similarity_score: score,
                })
                .collect();

            let response = SearchResponse {
                total_found: reviews_with_scores.len(),
                reviews: reviews_with_scores,
            };

            Ok(Json(ApiResponse::success(response)))
        }
        Err(e) => {
            tracing::error!("Failed to search reviews: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
