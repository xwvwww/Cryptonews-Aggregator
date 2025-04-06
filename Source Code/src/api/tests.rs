use std::env;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{coingecko, cryptopanic};
    use crate::models::news::NewsItem;

    #[tokio::test]
    async fn test_coingecko_api() {
        let api = coingecko::CoinGeckoApi::new();
        let result = api.fetch_news("BTC").await;
        
        assert!(result.is_ok(), "CoinGecko API request failed: {:?}", result.err());
        
        let news_items = result.unwrap();
        assert!(!news_items.is_empty(), "No news items returned from CoinGecko API");
        
        // Verify news item structure
        for item in news_items {
            assert!(!item.title.is_empty(), "News item title is empty");
            assert!(!item.url.is_empty(), "News item URL is empty");
            assert!(!item.source.is_empty(), "News item source is empty");
            assert_eq!(item.crypto_symbol, "BTC", "News item crypto symbol doesn't match");
        }
    }

    #[tokio::test]
    async fn test_cryptopanic_api() {
        let api_key = env::var("CRYPTOPANIC_API_KEY").unwrap_or_else(|_| "YOUR_API_KEY_HERE".to_string());
        let api = cryptopanic::CryptoPanicApi::new(api_key);
        
        // This test will be skipped if using the placeholder API key
        if api_key == "YOUR_API_KEY_HERE" {
            println!("Skipping CryptoPanic API test due to missing API key");
            return;
        }
        
        let result = api.fetch_news("ETH").await;
        
        assert!(result.is_ok(), "CryptoPanic API request failed: {:?}", result.err());
        
        let news_items = result.unwrap();
        assert!(!news_items.is_empty(), "No news items returned from CryptoPanic API");
        
        // Verify news item structure
        for item in news_items {
            assert!(!item.title.is_empty(), "News item title is empty");
            assert!(!item.url.is_empty(), "News item URL is empty");
            assert!(!item.source.is_empty(), "News item source is empty");
            assert_eq!(item.crypto_symbol, "ETH", "News item crypto symbol doesn't match");
        }
    }
}
