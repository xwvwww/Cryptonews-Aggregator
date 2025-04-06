use actix_web::{web, ResponseError, HttpResponse};
use thiserror::Error;
use std::fmt;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("API request failed: {0}")]
    ApiError(String),
    
    #[error("Failed to parse response: {0}")]
    ParseError(String),
    
    #[error("Rate limit exceeded")]
    RateLimitError,
    
    #[error("Internal server error: {0}")]
    InternalError(String),
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            AppError::ApiError("Request timed out".to_string())
        } else if err.is_status() {
            if let Some(status) = err.status() {
                if status.as_u16() == 429 {
                    return AppError::RateLimitError;
                }
            }
            AppError::ApiError(format!("API returned error status: {}", err))
        } else {
            AppError::ApiError(err.to_string())
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::ParseError(err.to_string())
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::RateLimitError => {
                HttpResponse::TooManyRequests().json(web::Json(ErrorResponse {
                    error: true,
                    message: self.to_string(),
                }))
            }
            AppError::ApiError(_) | AppError::ParseError(_) => {
                HttpResponse::BadGateway().json(web::Json(ErrorResponse {
                    error: true,
                    message: self.to_string(),
                }))
            }
            AppError::InternalError(_) => {
                HttpResponse::InternalServerError().json(web::Json(ErrorResponse {
                    error: true,
                    message: self.to_string(),
                }))
            }
        }
    }
}

#[derive(serde::Serialize)]
struct ErrorResponse {
    error: bool,
    message: String,
}
