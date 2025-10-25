//'api'
// import hashmap
use std::collections::HashMap;

fn GET(_params: HashMap<String, String>) -> (String, u16) {
    ("Hello from GET".to_string(), 500)
}