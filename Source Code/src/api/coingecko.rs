use async_trait::async_trait;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::Deserialize;

use crate::api::NewsApi;
use crate::models::news::NewsItem;
use crate::models::error::AppError;
use crate::services::rate_limiter::RATE_LIMITER;
use log::{error, info};

// CoinGecko API response structures
#[derive(Debug, Deserialize)]
struct CoinInfoResponse {
    id: String,
}

pub struct CoinGeckoApi {
    client: Client,
}

impl CoinGeckoApi {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

#[async_trait]
impl NewsApi for CoinGeckoApi {
    async fn fetch_news(&self, crypto_symbol: &str) -> Result<Vec<NewsItem>, Box<dyn std::error::Error>> {
        if !RATE_LIMITER.check_rate_limit("coingecko", 10) {
            return Err(Box::new(AppError::RateLimitError));
        }

        let coin_id = map_symbol_to_coingecko_id(crypto_symbol.to_lowercase());
        let coin_info_url = format!("https://api.coingecko.com/api/v3/coins/{}", coin_id);

        info!("Fetching CoinGecko data for: {}", coin_id);

        let response = self.client.get(&coin_info_url).send().await?;
        
        if response.status().is_success() {
            let coin_info: CoinInfoResponse = response.json().await?;
            info!("Coin exists: {}", coin_info.id);
        } else {
            let status = response.status();
            if status.as_u16() == 429 {
                return Err(Box::new(AppError::RateLimitError));
            } else if status.as_u16() == 404 {
                error!("CoinGecko API: Cryptocurrency '{}' not found", crypto_symbol);
                return Err(Box::new(AppError::ApiError(format!("Cryptocurrency '{}' not found", crypto_symbol))));
            }
            return Err(Box::new(AppError::ApiError(format!("API request failed with status: {}", status))));
        }

        // Generate mock news since CoinGecko free API has no news endpoint
        let now = Utc::now();
        let news_items: Vec<NewsItem> = (1..=5).map(|i| {
            let published_at = now - chrono::Duration::days(i);
            NewsItem {
                title: format!("Latest updates for {} cryptocurrency", crypto_symbol.to_uppercase()),
                url: format!("https://www.coingecko.com/en/coins/{}", coin_id),
                source: "CoinGecko".to_string(),
                published_at,
                summary: Some(format!("This is a simulated news item for {} from CoinGecko API.", crypto_symbol.to_uppercase())),
                image_url: Some("https://assets.coingecko.com/coins/images/1/large/bitcoin.png".to_string()),
                crypto_symbol: crypto_symbol.to_string(),
            }
        }).collect();

        Ok(news_items)
    }
}

// Helper function to map symbols to CoinGecko coin IDs
fn map_symbol_to_coingecko_id(symbol: String) -> String {
    match symbol.as_str() {
        "btc" => "bitcoin".to_string(),
        "eth" => "ethereum".to_string(),
        "sol" => "solana".to_string(),
        "doge" => "dogecoin".to_string(),
        "ada" => "cardano".to_string(),
        _ => symbol, // Fallback to original symbol
    }
}
