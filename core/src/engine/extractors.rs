use bytes::Bytes;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

/// Type-safe JSON body extractor
/// 
/// Usage:
/// ```rust
/// pub fn POST(body: Json<CreateTodoRequest>) -> Response {
///     let todo = body.0; // Access the inner type
///     // ...
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Json<T>(pub T);

impl<T: DeserializeOwned> Json<T> {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        serde_json::from_slice(bytes)
            .map(Json)
            .map_err(|e| format!("Failed to parse JSON body: {}", e))
    }
    
    pub fn into_inner(self) -> T {
        self.0
    }
}

/// Raw bytes extractor
#[derive(Debug, Clone)]
pub struct RawBody(pub Bytes);

impl RawBody {
    pub fn from_bytes(bytes: Bytes) -> Self {
        RawBody(bytes)
    }
    
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
    
    pub fn into_bytes(self) -> Bytes {
        self.0
    }
}

/// Form data extractor (application/x-www-form-urlencoded)
#[derive(Debug, Clone)]
pub struct Form(pub HashMap<String, String>);

impl Form {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        let body_str = std::str::from_utf8(bytes)
            .map_err(|e| format!("Invalid UTF-8 in form data: {}", e))?;
        
        let mut map = HashMap::new();
        for pair in body_str.split('&') {
            if let Some((key, value)) = pair.split_once('=') {
                let key = urlencoding::decode(key)
                    .map_err(|e| format!("Failed to decode form key: {}", e))?
                    .into_owned();
                let value = urlencoding::decode(value)
                    .map_err(|e| format!("Failed to decode form value: {}", e))?
                    .into_owned();
                map.insert(key, value);
            }
        }
        
        Ok(Form(map))
    }
    
    pub fn get(&self, key: &str) -> Option<&String> {
        self.0.get(key)
    }
}

/// Plain text body extractor
#[derive(Debug, Clone)]
pub struct Text(pub String);

impl Text {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        String::from_utf8(bytes.to_vec())
            .map(Text)
            .map_err(|e| format!("Failed to parse text body: {}", e))
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
