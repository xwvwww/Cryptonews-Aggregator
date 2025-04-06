use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use tera::Tera;

use crate::services::news_service::NewsService;

#[derive(Deserialize)]
pub struct SearchForm {
    crypto_symbol: String,
}

pub async fn index(tera: web::Data<Tera>) -> Result<HttpResponse> {
    let mut context = tera::Context::new();
    context.insert("title", "Cryptocurrency News Aggregator");
    
    let rendered = tera.render("index.html", &context)
        .map_err(|e| {
            log::error!("Template rendering error: {}", e);
            actix_web::error::ErrorInternalServerError(e)
        })?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

pub async fn search_news(
    form: web::Form<SearchForm>,
    tera: web::Data<Tera>,
    news_service: web::Data<NewsService>,
) -> Result<HttpResponse> {
    let crypto_symbol = form.crypto_symbol.trim().to_uppercase();
    
    if crypto_symbol.is_empty() {
        let mut context = tera::Context::new();
        context.insert("title", "Cryptocurrency News Aggregator");
        context.insert("error", "Please enter a cryptocurrency symbol");
        
        let rendered = tera.render("index.html", &context)
            .map_err(|e| {
                log::error!("Template rendering error: {}", e);
                actix_web::error::ErrorInternalServerError(e)
            })?;
        
        return Ok(HttpResponse::Ok().content_type("text/html").body(rendered));
    }
    
    // Fetch news
    let news_response = news_service.get_news(&crypto_symbol).await;
    
    let mut context = tera::Context::new();
    context.insert("title", "Cryptocurrency News Aggregator");
    context.insert("crypto_symbol", &crypto_symbol);
    context.insert("news", &news_response.items);
    context.insert("timestamp", &news_response.timestamp.to_rfc3339());
    
    let rendered = tera.render("results.html", &context)
        .map_err(|e| {
            log::error!("Template rendering error: {}", e);
            actix_web::error::ErrorInternalServerError(e)
        })?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}
