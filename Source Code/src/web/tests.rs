use actix_web::{test, App, http::StatusCode};
use crate::web::routes;

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn test_index_page() {
        let app = test::init_service(
            App::new().configure(routes::configure)
        ).await;
        
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_search_empty_symbol() {
        let app = test::init_service(
            App::new().configure(routes::configure)
        ).await;
        
        let req = test::TestRequest::post()
            .uri("/search")
            .set_form(("crypto_symbol", ""))
            .to_request();
            
        let resp = test::call_service(&app, req).await;
        
        assert_eq!(resp.status(), StatusCode::OK);
        
        // Should return the index page with an error message
        let body = test::read_body(resp).await;
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        
        assert!(body_str.contains("Please enter a cryptocurrency symbol"));
    }

    #[actix_web::test]
    async fn test_search_valid_symbol() {
        let app = test::init_service(
            App::new().configure(routes::configure)
        ).await;
        
        let req = test::TestRequest::post()
            .uri("/search")
            .set_form(("crypto_symbol", "BTC"))
            .to_request();
            
        let resp = test::call_service(&app, req).await;
        
        assert_eq!(resp.status(), StatusCode::OK);
        
        // Should return the results page
        let body = test::read_body(resp).await;
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        
        assert!(body_str.contains("Results for: BTC"));
    }
}
