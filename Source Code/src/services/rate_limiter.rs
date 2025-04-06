use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::sync::Mutex;

// Rate limiter for API calls
pub struct RateLimiter {
    // Maps API name to last request time and allowed requests per minute
    limits: Mutex<HashMap<String, (Instant, u32)>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            limits: Mutex::new(HashMap::new()),
        }
    }

    // Check if a request to the specified API is allowed
    // Returns true if allowed, false if rate limited
    pub fn check_rate_limit(&self, api_name: &str, requests_per_minute: u32) -> bool {
        let mut limits = self.limits.lock().unwrap();
        
        let now = Instant::now();
        
        if let Some(&(last_request, rpm)) = limits.get(api_name) {
            // If it's been less than a minute since the last request
            if now.duration_since(last_request) < Duration::from_secs(60) {
                // If we've exceeded the rate limit
                if rpm >= requests_per_minute {
                    return false;
                }
                
                // Update the request count
                limits.insert(api_name.to_string(), (last_request, rpm + 1));
            } else {
                // It's been more than a minute, reset the counter
                limits.insert(api_name.to_string(), (now, 1));
            }
        } else {
            // First request to this API
            limits.insert(api_name.to_string(), (now, 1));
        }
        
        true
    }
}

// Create a singleton rate limiter
lazy_static::lazy_static! {
    pub static ref RATE_LIMITER: RateLimiter = RateLimiter::new();
}
