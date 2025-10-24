//'api'
// Example API route showcasing new DX features
use std::collections::HashMap;

fn GET(params: &HashMap<String, String>) -> (String, u16) {
    // Simple JSON response example
    let response = format!(
        r#"{{"message": "Hello from the API!", "params": {}}}"#,
        serde_json::to_string(params).unwrap_or_else(|_| "{}".to_string())
    );
    (response, 200)
}

fn POST(_params: &HashMap<String, String>) -> (String, u16) {
    let response = r#"{"message": "POST request received", "status": "success"}"#;
    (response.to_string(), 201)
}
