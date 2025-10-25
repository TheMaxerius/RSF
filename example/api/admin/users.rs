// ✅ REAL EXAMPLE: Protected Admin Route with JWT
use std::collections::HashMap;
use serde::Serialize;
use core::engine::{JwtAuth, FrameworkError};
use once_cell::sync::Lazy;

// Global JWT instance (same secret as login route)
static JWT: Lazy<JwtAuth> = Lazy::new(|| {
    JwtAuth::new("your-secret-key-change-in-production".to_string())
});

#[derive(Serialize)]
struct User {
    id: String,
    username: String,
    email: String,
    role: String,
    created_at: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

pub fn GET(params: &HashMap<String, String>) -> (String, u16) {
    // ✅ REAL FEATURE: JWT Authentication Check
    // In production, extract from Authorization: Bearer <token> header
    let token = params.get("auth_token")
        .or_else(|| params.get("token"))
        .map(|s| s.as_str())
        .unwrap_or("");
    
    // Verify JWT token
    let user_id = match JWT.verify_token(token) {
        Ok(id) => id,
        Err(e) => {
            let error = ErrorResponse { 
                error: format!("Unauthorized: {}", e) 
            };
            return (serde_json::to_string(&error).unwrap(), 401);
        }
    };
    
    // ✅ Authorization: Check if user is admin
    // In production, check role from database
    if !user_id.contains("admin") {
        let error = ErrorResponse {
            error: "Forbidden: Admin access required".to_string()
        };
        return (serde_json::to_string(&error).unwrap(), 403);
    }
    
    println!("✅ Admin access granted for user: {}", user_id);
    
    // Fetch users from database (mocked)
    let users = vec![
        User {
            id: "usr_001".to_string(),
            username: "admin".to_string(),
            email: "admin@example.com".to_string(),
            role: "admin".to_string(),
            created_at: "2025-01-01T00:00:00Z".to_string(),
        },
        User {
            id: "usr_002".to_string(),
            username: "john".to_string(),
            email: "john@example.com".to_string(),
            role: "user".to_string(),
            created_at: "2025-02-15T10:30:00Z".to_string(),
        },
        User {
            id: "usr_003".to_string(),
            username: "jane".to_string(),
            email: "jane@example.com".to_string(),
            role: "user".to_string(),
            created_at: "2025-03-20T14:45:00Z".to_string(),
        },
    ];
    
    match serde_json::to_string(&users) {
        Ok(json) => (json, 200),
        Err(_) => {
            let error = ErrorResponse { error: "Internal server error".to_string() };
            (serde_json::to_string(&error).unwrap(), 500)
        }
    }
}
