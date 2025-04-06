# Cryptocurrency News Aggregator

A Rust-based service that collects and displays the latest cryptocurrency news from multiple sources. Users can enter a cryptocurrency name or symbol to retrieve recent news articles.

## Features

- Search news by cryptocurrency name or symbol
- Fetch data from multiple APIs (CryptoPanic and CoinGecko)
- Display news title, source, date, and summary
- Handle errors and API rate limits
- Simple web interface for user interaction

## Technology Stack

- Backend: Rust with Actix-web framework
- Frontend: HTML/CSS with Tera templates
- Data Sources: CryptoNews API, CoinGecko API

## Project Structure

```
crypto_news_aggregator/
├── src/
│   ├── api/              # API client implementations
│   │   ├── cryptopanic.rs
│   │   ├── coingecko.rs
│   │   ├── mod.rs
│   │   └── tests.rs
│   ├── models/           # Data models
│   │   ├── news.rs
│   │   ├── error.rs
│   │   └── mod.rs
│   ├── services/         # Business logic
│   │   ├── news_service.rs
│   │   ├── rate_limiter.rs
│   │   ├── mod.rs
│   │   └── tests.rs
│   ├── web/              # Web interface
│   │   ├── handlers.rs
│   │   ├── routes.rs
│   │   ├── mod.rs
│   │   ├── tests.rs
│   │   ├── templates/    # HTML templates
│   │   │   ├── index.html
│   │   │   └── results.html
│   │   └── static/       # Static assets
│   │       └── css/
│   │           └── style.css
│   └── main.rs           # Application entry point
└── Cargo.toml            # Project dependencies
```

## Setup and Installation

1. Install Rust and Cargo (if not already installed):
   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone the repository:
   ```
   git clone https://github.com/yourusername/crypto_news_aggregator.git
   cd crypto_news_aggregator
   ```

3. Set up environment variables (optional):
   ```
   export CRYPTOPANIC_API_KEY="your_api_key_here"
   export PORT="8080"
   ```

4. Build and run the application:
   ```
   cargo build --release
   cargo run --release
   ```

5. Open your browser and navigate to:
   ```
   http://localhost:8080
   ```

## API Keys

- CryptoPanic: Get your API key from [https://cryptopanic.com/developers/api/](https://cryptopanic.com/developers/api/)
- CoinGecko: Free tier doesn't require an API key

## Testing

Run the tests with:
```
cargo test
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
