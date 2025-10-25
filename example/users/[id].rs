//'api'
// Dynamic route example: /users/:id
use std::collections::HashMap;

fn GET(params: &HashMap<String, String>) -> (String, u16) {
    if let Some(id) = params.get("id") {
        let response = format!(
            r#"{{"user_id": "{}", "name": "User {}", "status": "active"}}"#,
            id, id
        );
        (response, 200)
    } else {
        (r#"{"error": "User ID required"}"#.to_string(), 400)
    }
}

fn DELETE(params: &HashMap<String, String>) -> (String, u16) {
    if let Some(id) = params.get("id") {
        let response = format!(r#"{{"deleted": true, "user_id": "{}"}}"#, id);
        (response, 200)
    } else {
        (r#"{"error": "User ID required"}"#.to_string(), 400)
    }
}
