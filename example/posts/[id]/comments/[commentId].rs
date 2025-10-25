//'api'
// Dynamic route with multiple params: /posts/:id/comments/:commentId
use std::collections::HashMap;

pub fn GET(params: &HashMap<String, String>) -> (String, u16) {
    let post_id = params.get("id").map(|s| s.as_str()).unwrap_or("unknown");
    let comment_id = params.get("commentId").map(|s| s.as_str()).unwrap_or("unknown");
    
    let response = format!(
        r#"{{"post_id": "{}", "comment_id": "{}", "content": "This is comment {} on post {}"}}"#,
        post_id, comment_id, comment_id, post_id
    );
    (response, 200)
}

pub fn DELETE(params: &HashMap<String, String>) -> (String, u16) {
    let post_id = params.get("id").map(|s| s.as_str()).unwrap_or("unknown");
    let comment_id = params.get("commentId").map(|s| s.as_str()).unwrap_or("unknown");
    
    let response = format!(
        r#"{{"deleted": true, "post_id": "{}", "comment_id": "{}"}}"#,
        post_id, comment_id
    );
    (response, 200)
}
