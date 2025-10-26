/// Middleware module for request/response processing
use bytes::Bytes;
use std::io::Write;
use flate2::Compression;
use flate2::write::GzEncoder;

/// Compression utilities
pub struct ResponseCompressor;

impl ResponseCompressor {
    /// Compress response body with gzip if beneficial
    #[inline]
    pub fn compress_gzip(body: &[u8], threshold: usize) -> Option<Bytes> {
        // Only compress if body is larger than threshold
        if body.len() < threshold {
            return None;
        }
        
        let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());
        if encoder.write_all(body).is_err() {
            return None;
        }
        
        match encoder.finish() {
            Ok(compressed) => {
                // Only use compressed if it's actually smaller
                if compressed.len() < body.len() {
                    Some(Bytes::from(compressed))
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }
    
    /// Check if client accepts gzip encoding
    #[inline]
    pub fn accepts_gzip(accept_encoding: Option<&str>) -> bool {
        accept_encoding
            .map(|s| s.contains("gzip"))
            .unwrap_or(false)
    }
}

/// Query string parser
pub struct QueryParser;

impl QueryParser {
    /// Parse query string into HashMap
    /// Example: "key1=value1&key2=value2" -> {key1: value1, key2: value2}
    #[inline]
    pub fn parse(query: &str) -> std::collections::HashMap<String, String> {
        if query.is_empty() {
            return std::collections::HashMap::new();
        }
        
        query
            .split('&')
            .filter_map(|pair| {
                let mut parts = pair.splitn(2, '=');
                match (parts.next(), parts.next()) {
                    (Some(key), Some(value)) => {
                        let key = percent_encoding::percent_decode_str(key)
                            .decode_utf8_lossy()
                            .to_string();
                        let value = percent_encoding::percent_decode_str(value)
                            .decode_utf8_lossy()
                            .to_string();
                        Some((key, value))
                    }
                    (Some(key), None) => {
                        let key = percent_encoding::percent_decode_str(key)
                            .decode_utf8_lossy()
                            .to_string();
                        Some((key, String::new()))
                    }
                    _ => None,
                }
            })
            .collect()
    }
    
    /// Extract query string from path
    #[inline]
    pub fn extract_from_path(path: &str) -> Option<&str> {
        path.split_once('?').map(|(_, query)| query)
    }
}

/// Rate limiter (simple token bucket implementation)
pub struct RateLimiter {
    // In a real implementation, this would use DashMap with timestamps
    // For now, just a placeholder structure
}

impl RateLimiter {
    pub fn new(_requests_per_second: u32) -> Self {
        Self {}
    }
    
    /// Check if request should be allowed (always true for now)
    #[inline]
    pub fn check_limit(&self, _client_ip: &str) -> bool {
        // TODO: Implement actual rate limiting with token bucket
        true
    }
}

/// CORS middleware
pub struct CorsMiddleware {
    allowed_origins: Vec<String>,
    allowed_methods: Vec<String>,
    allowed_headers: Vec<String>,
}

impl CorsMiddleware {
    pub fn new() -> Self {
        Self {
            allowed_origins: vec!["*".to_string()],
            allowed_methods: vec!["GET".to_string(), "POST".to_string(), "PUT".to_string(), "DELETE".to_string()],
            allowed_headers: vec!["Content-Type".to_string(), "Authorization".to_string()],
        }
    }
    
    pub fn allow_origin(mut self, origin: impl Into<String>) -> Self {
        self.allowed_origins = vec![origin.into()];
        self
    }
    
    pub fn allow_methods(mut self, methods: Vec<String>) -> Self {
        self.allowed_methods = methods;
        self
    }
    
    /// Generate CORS headers
    pub fn headers(&self) -> Vec<(String, String)> {
        vec![
            ("Access-Control-Allow-Origin".to_string(), self.allowed_origins.join(", ")),
            ("Access-Control-Allow-Methods".to_string(), self.allowed_methods.join(", ")),
            ("Access-Control-Allow-Headers".to_string(), self.allowed_headers.join(", ")),
        ]
    }
}

impl Default for CorsMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_query_parser() {
        let query = "name=John&age=30&city=New%20York";
        let parsed = QueryParser::parse(query);
        
        assert_eq!(parsed.get("name"), Some(&"John".to_string()));
        assert_eq!(parsed.get("age"), Some(&"30".to_string()));
        assert_eq!(parsed.get("city"), Some(&"New York".to_string()));
    }
    
    #[test]
    fn test_extract_query() {
        let path = "/api/users?id=123&filter=active";
        let query = QueryParser::extract_from_path(path);
        assert_eq!(query, Some("id=123&filter=active"));
    }
    
    #[test]
    fn test_compression() {
        let data = b"Hello World! ".repeat(100);
        let compressed = ResponseCompressor::compress_gzip(&data, 100);
        assert!(compressed.is_some());
        
        let compressed_data = compressed.unwrap();
        assert!(compressed_data.len() < data.len());
    }
}
