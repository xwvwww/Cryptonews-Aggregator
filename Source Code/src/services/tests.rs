use crate::services::rate_limiter::RATE_LIMITER;

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_rate_limiter() {
        // Test that requests within limits are allowed
        for _ in 0..5 {
            assert!(RATE_LIMITER.check_rate_limit("test_api", 5));
        }
        
        // The 6th request should be rate limited
        assert!(!RATE_LIMITER.check_rate_limit("test_api", 5));
        
        // Test that different APIs have separate rate limits
        assert!(RATE_LIMITER.check_rate_limit("another_api", 5));
        
        // Test that rate limits reset after a minute
        // Note: In a real test, we would mock time instead of sleeping
        println!("Waiting for rate limit to reset (this is a slow test)...");
        thread::sleep(Duration::from_secs(61));
        
        // After waiting, we should be able to make requests again
        assert!(RATE_LIMITER.check_rate_limit("test_api", 5));
    }
}
