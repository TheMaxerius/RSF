//'api'
// Example demonstrating query string parsing
use std::collections::HashMap;

pub fn GET(params: &HashMap<String, String>) -> (String, u16) {
    // In a real implementation, you'd parse from the actual request
    // For now, demonstrate the concept
    let response = r#"{"message": "Search endpoint", "tip": "Use ?q=search&limit=10"}"#;
    (response.to_string(), 200)
}
