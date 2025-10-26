// 'api'
// Example demonstrating improved developer experience with new helpers

use std::collections::HashMap;
use crate::engine::{Response, AppError, extract_param, Json};
use bytes::Bytes;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Serialize)]
struct User {
    id: usize,
    name: String,
    email: String,
}

/// GET /improved_dx - Simple demo with clean parameter extraction
pub async fn GET(params: &HashMap<String, String>) -> Response {
    // ✨ DX Improvement: Simple extraction with helpful error messages
    let user_id = match extract_param::<usize>(params, "id") {
        Ok(id) => id,
        Err(_) => {
            // Optional parameter - use default
            1
        }
    };
    
    Response::json(&serde_json::json!({
        "message": "Improved DX Demo",
        "features": [
            "✅ extract_param<T> - Type-safe parameter parsing with one line",
            "✅ AppError type - Result-based error handling with ? operator",
            "✅ Automatic error messages - No manual error construction",
            "✅ Helper functions - Reduced boilerplate for common patterns"
        ],
        "examples": {
            "parameter_extraction": "let id = extract_param::<usize>(params, \"id\")?;",
            "error_handling": "return Err(AppError::bad_request(\"Invalid input\"));",
            "json_parsing": "let data = Json::<T>::from_bytes(body).map_err(AppError::from)?;"
        },
        "user_id": user_id
    }), 200)
}

/// POST /improved_dx - Example with improved error handling
pub async fn POST(_params: &HashMap<String, String>, body: &Bytes) -> Response {
    // ✨ DX Improvement: Automatic error conversion with ?
    match handle_create_user(body) {
        Ok(user) => Response::json(&user, 201),
        Err(err) => err.into_response(),
    }
}

// Helper function using Result for cleaner error handling
fn handle_create_user(body: &Bytes) -> Result<User, AppError> {
    // ✨ DX Improvement: Json::from_bytes returns Result, use ? operator
    let request = Json::<CreateUser>::from_bytes(body)
        .map_err(|e| AppError::bad_request(format!("Invalid JSON: {}", e)))?;
    
    // ✨ DX Improvement: Use helper functions for validation
    if request.0.name.is_empty() {
        return Err(AppError::bad_request("Name cannot be empty"));
    }
    
    if !request.0.email.contains('@') {
        return Err(AppError::bad_request("Invalid email format"));
    }
    
    // Success case
    Ok(User {
        id: 123,
        name: request.0.name,
        email: request.0.email,
    })
}
