// GENERATED FILE - DO NOT EDIT

#[allow(non_snake_case)]
mod module__home_maxerius_Code_framework_test_core_src_engine_______example_main_rs {
    mod __orig {
        //'api'
        // import hashmap
        use std::collections::HashMap;
        
        pub(crate) fn GET(_params: HashMap<String, String>) -> (String, u16) {
            ("Hello from GET".to_string(), 500)
        }
    }
    // wrapper for GET that adapts (String,u16) -> Response
    pub fn GET(params: &std::collections::HashMap<String, String>) -> super::Response {
        let (s, status) = __orig::GET(params.clone());
        super::Response { status, body: s.into_bytes().into(), content_type: "text/plain; charset=utf-8", headers: Vec::new() }
    }
}

#[allow(non_snake_case)]
mod module__home_maxerius_Code_framework_test_core_src_engine_______example_api_rs {
    mod __orig {
        //'api'
        // Example API route showcasing new DX features
        use std::collections::HashMap;
        
        pub(crate) fn GET(params: &HashMap<String, String>) -> (String, u16) {
            // Simple JSON response example
            let response = format!(
                r#"{{"message": "Hello from the API!", "params": {}}}"#,
                serde_json::to_string(params).unwrap_or_else(|_| "{}".to_string())
            );
            (response, 200)
        }
        
        pub(crate) fn POST(_params: &HashMap<String, String>) -> (String, u16) {
            let response = r#"{"message": "POST request received", "status": "success"}"#;
            (response.to_string(), 201)
        }
    }
    // wrapper for GET that adapts (String,u16) -> Response
    pub fn GET(params: &std::collections::HashMap<String, String>) -> super::Response {
        let (s, status) = __orig::GET(params);
        super::Response { status, body: s.into_bytes().into(), content_type: "text/plain; charset=utf-8", headers: Vec::new() }
    }
    // wrapper for POST that adapts (String,u16) -> Response
    pub fn POST(params: &std::collections::HashMap<String, String>) -> super::Response {
        let (s, status) = __orig::POST(params);
        super::Response { status, body: s.into_bytes().into(), content_type: "text/plain; charset=utf-8", headers: Vec::new() }
    }
}

#[allow(non_snake_case)]
mod module__home_maxerius_Code_framework_test_core_src_engine_______example_users__id__rs {
    mod __orig {
        //'api'
        // Dynamic route example: /users/:id
        use std::collections::HashMap;
        
        pub(crate) fn GET(params: &HashMap<String, String>) -> (String, u16) {
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
        
        pub(crate) fn DELETE(params: &HashMap<String, String>) -> (String, u16) {
            if let Some(id) = params.get("id") {
                let response = format!(r#"{{"deleted": true, "user_id": "{}"}}"#, id);
                (response, 200)
            } else {
                (r#"{"error": "User ID required"}"#.to_string(), 400)
            }
        }
    }
    // wrapper for GET that adapts (String,u16) -> Response
    pub fn GET(params: &std::collections::HashMap<String, String>) -> super::Response {
        let (s, status) = __orig::GET(params);
        super::Response { status, body: s.into_bytes().into(), content_type: "text/plain; charset=utf-8", headers: Vec::new() }
    }
    // wrapper for DELETE that adapts (String,u16) -> Response
    pub fn DELETE(params: &std::collections::HashMap<String, String>) -> super::Response {
        let (s, status) = __orig::DELETE(params);
        super::Response { status, body: s.into_bytes().into(), content_type: "text/plain; charset=utf-8", headers: Vec::new() }
    }
}

#[allow(non_snake_case)]
mod module__home_maxerius_Code_framework_test_core_src_engine_______example_posts__id__comments__commentId__rs {
    mod __orig {
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
    }
    // wrapper for GET that adapts (String,u16) -> Response
    pub fn GET(params: &std::collections::HashMap<String, String>) -> super::Response {
        let (s, status) = __orig::GET(params);
        super::Response { status, body: s.into_bytes().into(), content_type: "text/plain; charset=utf-8", headers: Vec::new() }
    }
    // wrapper for DELETE that adapts (String,u16) -> Response
    pub fn DELETE(params: &std::collections::HashMap<String, String>) -> super::Response {
        let (s, status) = __orig::DELETE(params);
        super::Response { status, body: s.into_bytes().into(), content_type: "text/plain; charset=utf-8", headers: Vec::new() }
    }
}

use std::option::Option;

pub type Handler = fn(&std::collections::HashMap<String, String>) -> super::Response;

pub fn get_handler(route: &str, method: &str) -> Option<(Handler, std::collections::HashMap<String, String>)> {
    let route_normalized = route.trim_start_matches('/').trim_end_matches('/');
    let segments: Vec<&str> = if route_normalized.is_empty() { Vec::new() } else { route_normalized.split('/').collect() };

    // Match pattern: GET /main
    if method == "GET" && segments.len() == 1 {
        if segments.get(0) != Some(&"main") { /* skip */ } else
        {
            return Some((module__home_maxerius_Code_framework_test_core_src_engine_______example_main_rs::GET, std::collections::HashMap::new()));
        }
    }
    // Match pattern: GET /api
    if method == "GET" && segments.len() == 1 {
        if segments.get(0) != Some(&"api") { /* skip */ } else
        {
            return Some((module__home_maxerius_Code_framework_test_core_src_engine_______example_api_rs::GET, std::collections::HashMap::new()));
        }
    }
    // Match pattern: POST /api
    if method == "POST" && segments.len() == 1 {
        if segments.get(0) != Some(&"api") { /* skip */ } else
        {
            return Some((module__home_maxerius_Code_framework_test_core_src_engine_______example_api_rs::POST, std::collections::HashMap::new()));
        }
    }
    // Match pattern: GET /users/[id]
    if method == "GET" && segments.len() == 2 {
        if segments.get(0) != Some(&"users") { /* skip */ } else
        {
            let mut params = std::collections::HashMap::new();
            if let Some(val) = segments.get(1) {
                params.insert("id".to_string(), val.to_string());
            }
            return Some((module__home_maxerius_Code_framework_test_core_src_engine_______example_users__id__rs::GET, params));
        }
    }
    // Match pattern: DELETE /users/[id]
    if method == "DELETE" && segments.len() == 2 {
        if segments.get(0) != Some(&"users") { /* skip */ } else
        {
            let mut params = std::collections::HashMap::new();
            if let Some(val) = segments.get(1) {
                params.insert("id".to_string(), val.to_string());
            }
            return Some((module__home_maxerius_Code_framework_test_core_src_engine_______example_users__id__rs::DELETE, params));
        }
    }
    // Match pattern: GET /posts/[id]/comments/[commentId]
    if method == "GET" && segments.len() == 4 {
        if segments.get(0) != Some(&"posts") { /* skip */ } else
        if segments.get(2) != Some(&"comments") { /* skip */ } else
        {
            let mut params = std::collections::HashMap::new();
            if let Some(val) = segments.get(1) {
                params.insert("id".to_string(), val.to_string());
            }
            if let Some(val) = segments.get(3) {
                params.insert("commentId".to_string(), val.to_string());
            }
            return Some((module__home_maxerius_Code_framework_test_core_src_engine_______example_posts__id__comments__commentId__rs::GET, params));
        }
    }
    // Match pattern: DELETE /posts/[id]/comments/[commentId]
    if method == "DELETE" && segments.len() == 4 {
        if segments.get(0) != Some(&"posts") { /* skip */ } else
        if segments.get(2) != Some(&"comments") { /* skip */ } else
        {
            let mut params = std::collections::HashMap::new();
            if let Some(val) = segments.get(1) {
                params.insert("id".to_string(), val.to_string());
            }
            if let Some(val) = segments.get(3) {
                params.insert("commentId".to_string(), val.to_string());
            }
            return Some((module__home_maxerius_Code_framework_test_core_src_engine_______example_posts__id__comments__commentId__rs::DELETE, params));
        }
    }
    None
}
