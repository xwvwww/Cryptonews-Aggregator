use actix_files as fs;
use actix_web::{web, App, HttpServer, middleware};
use std::env;

mod api;
mod models;
mod services;
mod web_api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize environment variables
    dotenv::dotenv().ok();
    env_logger::init();

    // Debug log CryptoPanic API Key
if let Ok(api_key) = env::var("CRYPTOPANIC_API_KEY") {
    println!("CryptoPanic API Key Loaded: {}", api_key);
} else {
    println!("ERROR: CRYPTOPANIC_API_KEY not found!");
}
    
    // Set default port if not specified
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("0.0.0.0:{}", port);
    
    println!("Starting Cryptocurrency News Aggregator at: http://{}", bind_address);
    
    // Create the news service with all API clients
    let news_service = web::Data::new(api::create_news_service());
    
    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(news_service.clone())
            .wrap(middleware::Logger::default())
            // Serve static files
            .service(fs::Files::new("/static", "./static").show_files_listing())
            // API endpoints
            .service(
                web::scope("/api")
                    .route("/news/{symbol}", web::get().to(web_api::get_news))
            )
            // Serve index.html for all other routes
            .default_service(
                fs::Files::new("/", "./static")
                    .index_file("index.html")
            )
    })
    .bind(bind_address)?
    .run()
    .await
}
