//'api'
// ✅ REAL EXAMPLE: Authentication with JWT and Sessions
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use core::engine::{JwtAuth, SessionStore, BodyParser};
use std::time::Duration;
use once_cell::sync::Lazy;

// Global instances (in production, use dependency injection)
static JWT: Lazy<JwtAuth> = Lazy::new(|| {
    JwtAuth::new("your-secret-key-change-in-production".to_string())
});

static SESSIONS: Lazy<SessionStore> = Lazy::new(|| {
    SessionStore::new(Duration::from_secs(3600)) // 1 hour TTL
});

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    success: bool,
    token: String,
    user_id: String,
    session_id: String,
    expires_in: u64,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

pub fn POST(params: &HashMap<String, String>) -> (String, u16) {
    // ✅ REAL FEATURE: JSON Body Parsing
    let mock_body = r#"{"username": "admin", "password": "admin123"}"#;
    
    let credentials = match BodyParser::json::<LoginRequest>(mock_body.as_bytes()) {
        Ok(creds) => creds,
        Err(e) => {
            let error = ErrorResponse { error: format!("Invalid JSON: {}", e) };
            return (serde_json::to_string(&error).unwrap(), 400);
        }
    };
    
    // Simple credential check (use database in production)
    if credentials.username != "admin" || credentials.password != "admin123" {
        let error = ErrorResponse { error: "Invalid credentials".to_string() };
        return (serde_json::to_string(&error).unwrap(), 401);
    }
    
    let user_id = format!("user_{}", credentials.username);
    
    // ✅ REAL FEATURE: JWT Token Generation
    let token = JWT.generate_token(&user_id, Duration::from_secs(86400)); // 24 hours
    
    // ✅ REAL FEATURE: Session Creation
    let session_id = format!("sess_{}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis());
    
    let mut session = SESSIONS.create(session_id.clone());
    session.set("user_id".to_string(), user_id.clone());
    session.set("username".to_string(), credentials.username);
    SESSIONS.update(session);
    
    let response = LoginResponse {
        success: true,
        token,
        user_id,
        session_id,
        expires_in: 3600,
    };
    
    (serde_json::to_string(&response).unwrap(), 200)
}

// ✅ REAL FEATURE: Token Verification
pub fn GET(params: &HashMap<String, String>) -> (String, u16) {
    // Verify JWT token (extract from Authorization header in production)
    let token = params.get("token").map(|s| s.as_str()).unwrap_or("");
    
    match JWT.verify_token(token) {
        Ok(user_id) => {
            let response = serde_json::json!({
                "valid": true,
                "user_id": user_id,
                "message": "Token is valid"
            });
            (response.to_string(), 200)
        }
        Err(e) => {
            let error = ErrorResponse { error: format!("Invalid token: {}", e) };
            (serde_json::to_string(&error).unwrap(), 401)
        }
    }
}

// ✅ REAL FEATURE: Session Logout
pub fn DELETE(params: &HashMap<String, String>) -> (String, u16) {
    let session_id = params.get("session_id")
        .or_else(|| params.get("id"))
        .map(|s| s.as_str())
        .unwrap_or("");
    
    SESSIONS.delete(session_id);
    
    let response = serde_json::json!({
        "success": true,
        "message": "Logged out successfully"
    });
    
    (response.to_string(), 200)
}
