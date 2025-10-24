// Better error handling for the framework
use thiserror::Error;
use std::fmt;

#[derive(Error, Debug)]
pub enum FrameworkError {
    #[error("Route not found: {path}")]
    RouteNotFound { path: String },
    
    #[error("Invalid method: {method}")]
    InvalidMethod { method: String },
    
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Internal server error: {0}")]
    InternalError(String),
}

impl FrameworkError {
    pub fn status_code(&self) -> u16 {
        match self {
            Self::RouteNotFound { .. } => 404,
            Self::InvalidMethod { .. } => 405,
            Self::ParseError(_) => 400,
            Self::IoError(_) => 500,
            Self::InternalError(_) => 500,
        }
    }

    pub fn to_json(&self) -> String {
        format!(
            r#"{{"error": "{}", "code": {}}}"#,
            self.to_string().replace('"', r#"\""#),
            self.status_code()
        )
    }
}

pub type Result<T> = std::result::Result<T, FrameworkError>;
