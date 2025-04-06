use actix_files as fs;
use actix_web::{web, HttpResponse};
use tera::Tera;

use crate::api::create_news_service;

pub fn configure(cfg: &mut web::ServiceConfig) {
    // Initialize template engine
    let mut tera = Tera::new("src/web/templates/**/*").expect("Failed to initialize template engine");
    tera.autoescape_on(vec!["html"]);
    
    // Register static files
    cfg.service(fs::Files::new("/static", "src/web/static").show_files_listing());
    
    // Create and register the news service
    let news_service = web::Data::new(create_news_service());
    
    // Register routes
    cfg.app_data(web::Data::new(tera))
       .app_data(news_service)
       .route("/", web::get().to(crate::web::handlers::index))
       .route("/search", web::post().to(crate::web::handlers::search_news));
}
