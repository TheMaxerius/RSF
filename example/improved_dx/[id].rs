// 'api'
// Example showing improved path parameter extraction

use std::collections::HashMap;
use crate::engine::{Response, extract_param};
use serde::Serialize;

#[derive(Serialize)]
struct UserDetail {
    id: usize,
    name: String,
    email: String,
    status: String,
}

/// GET /improved_dx/:id - Clean path parameter extraction
pub async fn GET(params: &HashMap<String, String>) -> Response {
    // ✨ DX Improvement: One-line parameter extraction with automatic error handling
    let user_id = match extract_param::<usize>(params, "id") {
        Ok(id) => id,
        Err(e) => {
            return Response::json(&serde_json::json!({
                "error": e
            }), 400);
        }
    };
    
    // Simulate database lookup
    let user = UserDetail {
        id: user_id,
        name: format!("User {}", user_id),
        email: format!("user{}@example.com", user_id),
        status: "active".to_string(),
    };
    
    Response::json(&user, 200)
}

/// DELETE /improved_dx/:id - Example with custom error handling
pub async fn DELETE(params: &HashMap<String, String>) -> Response {
    let user_id = match extract_param::<usize>(params, "id") {
        Ok(id) => id,
        Err(e) => {
            return Response::json(&serde_json::json!({
                "error": format!("Invalid user ID: {}", e)
            }), 400);
        }
    };
    
    // Simulate deletion
    Response::json(&serde_json::json!({
        "message": format!("User {} deleted successfully", user_id),
        "id": user_id
    }), 200)
}
