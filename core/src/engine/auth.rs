/// Authentication and session management
use dashmap::DashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use serde::{Deserialize, Serialize};

/// Session data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub user_id: Option<String>,
    pub data: std::collections::HashMap<String, String>,
    pub created_at: SystemTime,
    pub expires_at: SystemTime,
}

impl Session {
    pub fn new(id: String, ttl: Duration) -> Self {
        let now = SystemTime::now();
        Self {
            id,
            user_id: None,
            data: std::collections::HashMap::new(),
            created_at: now,
            expires_at: now + ttl,
        }
    }
    
    pub fn is_expired(&self) -> bool {
        SystemTime::now() > self.expires_at
    }
    
    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }
    
    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
}

/// Session store
pub struct SessionStore {
    sessions: Arc<DashMap<String, Session>>,
    default_ttl: Duration,
}

impl SessionStore {
    pub fn new(default_ttl: Duration) -> Self {
        Self {
            sessions: Arc::new(DashMap::new()),
            default_ttl,
        }
    }
    
    pub fn create(&self, session_id: String) -> Session {
        let session = Session::new(session_id.clone(), self.default_ttl);
        self.sessions.insert(session_id, session.clone());
        session
    }
    
    pub fn get(&self, session_id: &str) -> Option<Session> {
        self.sessions.get(session_id).map(|s| s.clone())
    }
    
    pub fn update(&self, session: Session) {
        self.sessions.insert(session.id.clone(), session);
    }
    
    pub fn delete(&self, session_id: &str) {
        self.sessions.remove(session_id);
    }
    
    pub fn cleanup_expired(&self) {
        self.sessions.retain(|_, session| !session.is_expired());
    }
}

impl Clone for SessionStore {
    fn clone(&self) -> Self {
        Self {
            sessions: Arc::clone(&self.sessions),
            default_ttl: self.default_ttl,
        }
    }
}

/// JWT Token utilities
pub struct JwtAuth {
    secret: String,
}

impl JwtAuth {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
    
    /// Generate a simple JWT-like token (simplified, use a real JWT lib in production)
    pub fn generate_token(&self, user_id: &str, expiry: Duration) -> String {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let exp = now + expiry.as_secs();
        
        // Simple base64 encoding (use proper JWT in production!)
        let payload = format!("{{\"user_id\":\"{}\",\"exp\":{}}}", user_id, exp);
        let signature = self.sign(&payload);
        
        format!("{}.{}", 
            base64_encode(&payload),
            base64_encode(&signature)
        )
    }
    
    /// Verify token (simplified)
    pub fn verify_token(&self, token: &str) -> Result<String, String> {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 2 {
            return Err("Invalid token format".to_string());
        }
        
        let payload = base64_decode(parts[0])
            .ok_or("Invalid base64")?;
        let signature = base64_decode(parts[1])
            .ok_or("Invalid signature")?;
        
        let expected_sig = self.sign(&payload);
        if signature != expected_sig {
            return Err("Invalid signature".to_string());
        }
        
        // Parse user_id from payload
        if let Ok(data) = serde_json::from_str::<serde_json::Value>(&payload) {
            if let Some(user_id) = data.get("user_id").and_then(|v| v.as_str()) {
                return Ok(user_id.to_string());
            }
        }
        
        Err("Invalid payload".to_string())
    }
    
    fn sign(&self, data: &str) -> String {
        // Simple HMAC-like signature (use proper HMAC in production!)
        format!("{}{}", data, self.secret)
            .bytes()
            .fold(0u64, |acc, b| acc.wrapping_add(b as u64))
            .to_string()
    }
}

fn base64_encode(data: &str) -> String {
    const TABLE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let bytes = data.as_bytes();
    let mut result = String::new();
    
    for chunk in bytes.chunks(3) {
        let b1 = chunk[0];
        let b2 = chunk.get(1).copied().unwrap_or(0);
        let b3 = chunk.get(2).copied().unwrap_or(0);
        
        result.push(TABLE[(b1 >> 2) as usize] as char);
        result.push(TABLE[(((b1 & 0x03) << 4) | (b2 >> 4)) as usize] as char);
        result.push(if chunk.len() > 1 { TABLE[(((b2 & 0x0f) << 2) | (b3 >> 6)) as usize] as char } else { '=' });
        result.push(if chunk.len() > 2 { TABLE[(b3 & 0x3f) as usize] as char } else { '=' });
    }
    
    result
}

fn base64_decode(data: &str) -> Option<String> {
    // Simplified decoder
    let decoded: Vec<u8> = data.bytes()
        .filter(|&b| b != b'=')
        .map(|b| match b {
            b'A'..=b'Z' => b - b'A',
            b'a'..=b'z' => b - b'a' + 26,
            b'0'..=b'9' => b - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            _ => 0,
        })
        .collect();
    
    String::from_utf8(decoded).ok()
}

/// Basic authentication
pub struct BasicAuth;

impl BasicAuth {
    /// Parse basic auth header
    pub fn parse(auth_header: &str) -> Option<(String, String)> {
        if !auth_header.starts_with("Basic ") {
            return None;
        }
        
        let encoded = &auth_header[6..];
        let decoded = base64_decode(encoded)?;
        
        let parts: Vec<&str> = decoded.splitn(2, ':').collect();
        if parts.len() == 2 {
            Some((parts[0].to_string(), parts[1].to_string()))
        } else {
            None
        }
    }
    
    /// Create basic auth header value
    pub fn create(username: &str, password: &str) -> String {
        format!("Basic {}", base64_encode(&format!("{}:{}", username, password)))
    }
}
