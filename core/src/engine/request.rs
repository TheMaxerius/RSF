/// Request body parsing utilities
use bytes::Bytes;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

/// Request body parser
pub struct BodyParser;

impl BodyParser {
    /// Parse JSON body
    pub fn json<T: DeserializeOwned>(body: &[u8]) -> Result<T, String> {
        serde_json::from_slice(body).map_err(|e| format!("JSON parse error: {}", e))
    }
    
    /// Parse form data (application/x-www-form-urlencoded)
    pub fn form(body: &[u8]) -> Result<HashMap<String, String>, String> {
        let body_str = std::str::from_utf8(body)
            .map_err(|e| format!("UTF-8 decode error: {}", e))?;
        
        Ok(crate::engine::middleware::QueryParser::parse(body_str))
    }
    
    /// Get raw body as bytes
    #[inline]
    pub fn bytes(body: &[u8]) -> Bytes {
        Bytes::copy_from_slice(body)
    }
    
    /// Get body as string
    pub fn text(body: &[u8]) -> Result<String, String> {
        std::str::from_utf8(body)
            .map(|s| s.to_string())
            .map_err(|e| format!("UTF-8 decode error: {}", e))
    }
}

/// Full request context with body
#[derive(Debug, Clone)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Bytes,
    pub params: HashMap<String, String>,
}

impl Request {
    pub fn new(method: String, path: String) -> Self {
        Self {
            method,
            path,
            headers: HashMap::new(),
            body: Bytes::new(),
            params: HashMap::new(),
        }
    }
    
    pub fn with_body(mut self, body: Bytes) -> Self {
        self.body = body;
        self
    }
    
    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }
    
    pub fn with_params(mut self, params: HashMap<String, String>) -> Self {
        self.params = params;
        self
    }
    
    /// Parse JSON body
    pub fn json<T: DeserializeOwned>(&self) -> Result<T, String> {
        BodyParser::json(&self.body)
    }
    
    /// Parse form data
    pub fn form(&self) -> Result<HashMap<String, String>, String> {
        BodyParser::form(&self.body)
    }
    
    /// Get header value
    pub fn header(&self, name: &str) -> Option<&String> {
        self.headers.get(name)
    }
    
    /// Get query parameter
    pub fn query(&self, name: &str) -> Option<String> {
        crate::engine::middleware::QueryParser::extract_from_path(&self.path)
            .map(|q| crate::engine::middleware::QueryParser::parse(q))
            .and_then(|map| map.get(name).cloned())
    }
    
    /// Get route parameter
    pub fn param(&self, name: &str) -> Option<&String> {
        self.params.get(name)
    }
}
