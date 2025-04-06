use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsItem {
    pub title: String,
    pub url: String,
    pub source: String,
    pub published_at: DateTime<Utc>,
    pub summary: Option<String>,
    pub image_url: Option<String>,
    pub crypto_symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsResponse {
    pub items: Vec<NewsItem>,
    pub crypto_symbol: String,
    pub timestamp: DateTime<Utc>,
}
