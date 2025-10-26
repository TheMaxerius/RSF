use crate::engine::Response;
use std::fmt;

/// Application error type with automatic Response conversion
#[derive(Debug)]
pub struct AppError {
    pub message: String,
    pub status: u16,
}

impl AppError {
    pub fn new(message: impl Into<String>, status: u16) -> Self {
        Self {
            message: message.into(),
            status,
        }
    }
    
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(message, 400)
    }
    
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(message, 401)
    }
    
    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::new(message, 403)
    }
    
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(message, 404)
    }
    
    pub fn internal_server_error(message: impl Into<String>) -> Self {
        Self::new(message, 500)
    }
    
    /// Convert to Response
    pub fn into_response(self) -> Response {
        Response::json(&serde_json::json!({
            "error": self.message
        }), self.status)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AppError {}

impl From<String> for AppError {
    fn from(s: String) -> Self {
        Self::bad_request(s)
    }
}

impl From<&str> for AppError {
    fn from(s: &str) -> Self {
        Self::bad_request(s)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        Self::bad_request(format!("JSON error: {}", e))
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::bad_request(format!("Parse error: {}", e))
    }
}
