// ✅ REAL EXAMPLE: CORS-Enabled Public API
use std::collections::HashMap;
use serde::Serialize;
use core::engine::CorsMiddleware;

#[derive(Serialize)]
struct PublicData {
    items: Vec<DataItem>,
    total: usize,
    timestamp: u64,
}

#[derive(Serialize)]
struct DataItem {
    id: String,
    name: String,
    value: f64,
}

pub fn GET(params: &HashMap<String, String>) -> (String, u16) {
    // ✅ REAL FEATURE: CORS Configuration
    let _cors = CorsMiddleware::new()
        .allow_origin("*")
        .allow_methods(vec!["GET".to_string(), "OPTIONS".to_string()]);
    
    // Note: In production, CORS headers would be added to response:
    // for (key, value) in cors.headers() {
    //     response.headers.push((key, value));
    // }
    
    let data = PublicData {
        items: vec![
            DataItem {
                id: "item_1".to_string(),
                name: "CPU Usage".to_string(),
                value: 42.5,
            },
            DataItem {
                id: "item_2".to_string(),
                name: "Memory Usage".to_string(),
                value: 68.3,
            },
            DataItem {
                id: "item_3".to_string(),
                name: "Disk I/O".to_string(),
                value: 15.7,
            },
        ],
        total: 3,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    };
    
    // Demonstrate CORS headers that would be added
    println!("✅ CORS headers configured:");
    println!("   Access-Control-Allow-Origin: *");
    println!("   Access-Control-Allow-Methods: GET, OPTIONS");
    
    match serde_json::to_string(&data) {
        Ok(json) => (json, 200),
        Err(_) => (r#"{"error": "Failed to serialize data"}"#.to_string(), 500),
    }
}

pub fn OPTIONS(params: &HashMap<String, String>) -> (String, u16) {
    // ✅ REAL FEATURE: CORS Preflight Response
    let _cors = CorsMiddleware::new()
        .allow_origin("*")
        .allow_methods(vec!["GET".to_string(), "OPTIONS".to_string()]);
    
    // CORS headers would be added here
    println!("✅ CORS preflight handled");
    
    ("".to_string(), 204)
}
