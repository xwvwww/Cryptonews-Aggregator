use actix_web::{web, HttpResponse, Result};
use serde::Serialize;

use crate::services::news_service::NewsService;

#[derive(Serialize)]
struct ApiResponse {
    items: Vec<crate::models::news::NewsItem>,
    timestamp: chrono::DateTime<chrono::Utc>,
}

pub async fn get_news(
    path: web::Path<String>,
    news_service: web::Data<NewsService>,
) -> Result<HttpResponse> {
    let crypto_symbol = path.into_inner().to_uppercase();
    
    // Fetch news using the news service
    let news_response = news_service.get_news(&crypto_symbol).await;
    
    // Convert to API response format
    let api_response = ApiResponse {
        items: news_response.items,
        timestamp: news_response.timestamp,
    };
    
    // Return JSON response
    Ok(HttpResponse::Ok().json(api_response))
}
