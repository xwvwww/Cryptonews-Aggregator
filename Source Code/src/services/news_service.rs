use crate::api::NewsApi;
use chrono::{DateTime, Utc};
use futures::future::join_all;
use std::sync::Arc;

pub struct NewsService {
    apis: Vec<Arc<dyn NewsApi + Send + Sync>>,
}

impl NewsService {
    pub fn new(apis: Vec<Arc<dyn NewsApi + Send + Sync>>) -> Self {
        Self { apis }
    }

    pub async fn get_news(&self, crypto_symbol: &str) -> NewsResponse {
        let futures = self.apis
            .iter()
            .map(|api| {
                let api_clone = Arc::clone(api);
                let symbol = crypto_symbol.to_string();
                tokio::spawn(async move {
                    match api_clone.fetch_news(&symbol).await {
                        Ok(news) => news,
                        Err(e) => {
                            log::error!("Error fetching news: {}", e);
                            vec![]
                        }
                    }
                })
            })
            .collect::<Vec<_>>();

        let results = join_all(futures).await;
        
        let mut all_news = Vec::new();
        for result in results {
            if let Ok(news) = result {
                all_news.extend(news);
            }
        }

        // Sort by published date (newest first)
        all_news.sort_by(|a, b| b.published_at.cmp(&a.published_at));

        NewsResponse {
            items: all_news,
            crypto_symbol: crypto_symbol.to_string(),
            timestamp: Utc::now(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct NewsResponse {
    pub items: Vec<crate::models::news::NewsItem>,
    pub crypto_symbol: String,
    pub timestamp: DateTime<Utc>,
}
