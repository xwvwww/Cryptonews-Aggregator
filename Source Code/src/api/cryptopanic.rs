use async_trait::async_trait;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::Deserialize;

use crate::api::NewsApi;
use crate::models::news::NewsItem;
use crate::models::error::AppError;
use crate::services::rate_limiter::RATE_LIMITER;
use log::{error, info};

// CryptoPanic API response structures
#[derive(Debug, Deserialize)]
struct CryptoPanicResponse {
    results: Vec<CryptoPanicResult>,
}

#[derive(Debug, Deserialize)]
struct CryptoPanicResult {
    title: String,
    url: String,
    published_at: String,
    source: CryptoPanicSource,
    metadata: Option<CryptoPanicMetadata>,
}

#[derive(Debug, Deserialize)]
struct CryptoPanicSource {
    title: String,
}

#[derive(Debug, Deserialize)]
struct CryptoPanicMetadata {
    image: Option<String>,
    description: Option<String>,
}

pub struct CryptoPanicApi {
    client: Client,
    api_key: String,
}

impl CryptoPanicApi {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }
}

#[async_trait]
impl NewsApi for CryptoPanicApi {
    async fn fetch_news(&self, crypto_symbol: &str) -> Result<Vec<NewsItem>, Box<dyn std::error::Error>> {
        if !RATE_LIMITER.check_rate_limit("cryptopanic", 5) {
            return Err(Box::new(AppError::RateLimitError));
        }

        let url = format!(
            "https://cryptopanic.com/api/v1/posts/?auth_token={}&currencies={}&public=true",
            self.api_key, crypto_symbol
        );

        info!("Fetching CryptoPanic news: {}", url);

        let response = self.client.get(&url).send().await?;
        let status = response.status();
        
        if status.is_success() {
            let cryptopanic_response: CryptoPanicResponse = response.json().await?;
            let news_items = cryptopanic_response.results.into_iter().map(|result| {
                let published_at = DateTime::parse_from_rfc3339(&result.published_at)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now());

                NewsItem {
                    title: result.title,
                    url: result.url,
                    source: result.source.title,
                    published_at,
                    summary: result.metadata.as_ref().and_then(|m| m.description.clone()),
                    image_url: result.metadata.as_ref().and_then(|m| m.image.clone()),
                    crypto_symbol: crypto_symbol.to_string(),
                }
            }).collect();

            Ok(news_items)
        } else {
            if status.as_u16() == 429 {
                error!("CryptoPanic API rate limit exceeded");
                return Err(Box::new(AppError::RateLimitError));
            } else if status.as_u16() == 404 {
                error!("CryptoPanic API: No news found for '{}'", crypto_symbol);
                return Err(Box::new(AppError::ApiError(format!("No news found for '{}'", crypto_symbol))));
            }
            return Err(Box::new(AppError::ApiError(format!("API request failed with status: {}", status))));
        }
    }
}
