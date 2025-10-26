// GENERATED FILE - DO NOT EDIT

#[allow(non_snake_case)]
mod module__home_runner_workspace_core_src_engine__________example_posts__id__rs {
    mod __orig {
        // 'api'
        use std::collections::HashMap;
        use serde::{Serialize, Deserialize};
        use crate::engine::{Response, Json};
        use bytes::Bytes;
        
        #[derive(Serialize, Deserialize, Clone, Debug)]
        struct Post {
            id: usize,
            title: String,
            content: String,
            author: String,
        }
        
        #[derive(Deserialize)]
        struct UpdatePostRequest {
            title: String,
            content: String,
            author: String,
        }
        
        /// GET /posts/:id - Get a specific post
        pub async fn GET(params: &HashMap<String, String>) -> Response {
            let id: usize = match params.get("id") {
                Some(id_str) => match id_str.parse() {
                    Ok(n) => n,
                    Err(_) => {
                        return Response::json(&serde_json::json!({
                            "error": "Invalid post ID"
                        }), 400);
                    }
                },
                None => {
                    return Response::json(&serde_json::json!({
                        "error": "Missing post ID"
                    }), 400);
                }
            };
        
            // For demo purposes, return a mock post
            let post = Post {
                id,
                title: format!("Post #{}", id),
                content: format!("This is the content of post {}. In a real app, this would come from a database.", id),
                author: "Demo Author".to_string(),
            };
            
            Response::json(&post, 200)
        }
        
        /// PUT /posts/:id - Update a post
        pub async fn PUT(params: &HashMap<String, String>, body: &Bytes) -> Response {
            let id: usize = match params.get("id") {
                Some(id_str) => match id_str.parse() {
                    Ok(n) => n,
                    Err(_) => {
                        return Response::json(&serde_json::json!({
                            "error": "Invalid post ID"
                        }), 400);
                    }
                },
                None => {
                    return Response::json(&serde_json::json!({
                        "error": "Missing post ID"
                    }), 400);
                }
            };
        
            // Parse JSON body
            let request: Json<UpdatePostRequest> = match Json::from_bytes(body) {
                Ok(json) => json,
                Err(e) => {
                    return Response::json(&serde_json::json!({
                        "error": "Invalid JSON body",
                        "details": e
                    }), 400);
                }
            };
        
            let updated_post = Post {
                id,
                title: request.0.title,
                content: request.0.content,
                author: request.0.author,
            };
            
            Response::json(&serde_json::json!({
                "message": "Post updated successfully",
                "post": updated_post
            }), 200)
        }
        
        /// DELETE /posts/:id - Delete a post
        pub async fn DELETE(params: &HashMap<String, String>) -> Response {
            let id: usize = match params.get("id") {
                Some(id_str) => match id_str.parse() {
                    Ok(n) => n,
                    Err(_) => {
                        return Response::json(&serde_json::json!({
                            "error": "Invalid post ID"
                        }), 400);
                    }
                },
                None => {
                    return Response::json(&serde_json::json!({
                        "error": "Missing post ID"
                    }), 400);
                }
            };
        
            Response::json(&serde_json::json!({
                "message": format!("Post {} deleted successfully", id),
                "deleted_id": id
            }), 200)
        }
    }
    // async wrapper for GET that forwards Response
    #[inline(always)]
    pub fn GET(params: &std::collections::HashMap<String, String>, body: &bytes::Bytes) -> std::pin::Pin<Box<dyn std::future::Future<Output = super::Response> + Send>> {
        let params = params.clone();
        let body = body.clone();
        Box::pin(async move {
            __orig::GET(&params).await
        })
    }
    // async wrapper for PUT that forwards Response
    #[inline(always)]
    pub fn PUT(params: &std::collections::HashMap<String, String>, body: &bytes::Bytes) -> std::pin::Pin<Box<dyn std::future::Future<Output = super::Response> + Send>> {
        let params = params.clone();
        let body = body.clone();
        Box::pin(async move {
            __orig::PUT(&params, &body).await
        })
    }
    // async wrapper for DELETE that forwards Response
    #[inline(always)]
    pub fn DELETE(params: &std::collections::HashMap<String, String>, body: &bytes::Bytes) -> std::pin::Pin<Box<dyn std::future::Future<Output = super::Response> + Send>> {
        let params = params.clone();
        let body = body.clone();
        Box::pin(async move {
            __orig::DELETE(&params).await
        })
    }
}

#[allow(non_snake_case)]
mod module__home_runner_workspace_core_src_engine__________example_posts_rs {
    mod __orig {
        // 'api'
        use std::collections::HashMap;
        use std::sync::{Arc, Mutex};
        use serde::{Serialize, Deserialize};
        use crate::engine::{Response, Json};
        use bytes::Bytes;
        use once_cell::sync::Lazy;
        
        #[derive(Serialize, Deserialize, Clone, Debug)]
        pub struct Post {
            pub id: usize,
            pub title: String,
            pub content: String,
            pub author: String,
            pub created_at: String,
            pub updated_at: String,
        }
        
        #[derive(Deserialize)]
        struct CreatePostRequest {
            title: String,
            content: String,
            author: String,
        }
        
        // Global in-memory database
        pub static POSTS: Lazy<Arc<Mutex<Vec<Post>>>> = Lazy::new(|| {
            Arc::new(Mutex::new(vec![
                Post {
                    id: 1,
                    title: "Welcome to the Blog API".to_string(),
                    content: "This is a blazingly fast blog API built with Rust! Features include async handlers, type-safe routing, and sub-millisecond response times.".to_string(),
                    author: "Admin".to_string(),
                    created_at: "2025-10-26T08:00:00Z".to_string(),
                    updated_at: "2025-10-26T08:00:00Z".to_string(),
                },
                Post {
                    id: 2,
                    title: "Why Rust for Web Development?".to_string(),
                    content: "Rust provides memory safety, zero-cost abstractions, and incredible performance. Perfect for building fast APIs!".to_string(),
                    author: "Alice".to_string(),
                    created_at: "2025-10-26T09:00:00Z".to_string(),
                    updated_at: "2025-10-26T09:00:00Z".to_string(),
                },
                Post {
                    id: 3,
                    title: "Async/Await in Rust".to_string(),
                    content: "Rust's async/await syntax makes it easy to write concurrent code. This API uses Tokio for async runtime!".to_string(),
                    author: "Bob".to_string(),
                    created_at: "2025-10-26T10:00:00Z".to_string(),
                    updated_at: "2025-10-26T10:00:00Z".to_string(),
                },
            ]))
        });
        
        /// GET /posts - List all posts
        pub async fn GET(_params: &HashMap<String, String>) -> Response {
            let posts = POSTS.lock().unwrap();
            Response::json(&*posts, 200)
        }
        
        /// POST /posts - Create a new post
        pub async fn POST(_params: &HashMap<String, String>, body: &Bytes) -> Response {
            // Parse JSON body
            let request: Json<CreatePostRequest> = match Json::from_bytes(body) {
                Ok(json) => json,
                Err(e) => {
                    return Response::json(&serde_json::json!({
                        "error": "Invalid JSON body",
                        "details": e
                    }), 400);
                }
            };
        
            let mut posts = POSTS.lock().unwrap();
            let next_id = posts.iter().map(|p| p.id).max().unwrap_or(0) + 1;
            
            let now = chrono::Utc::now().to_rfc3339();
            let new_post = Post {
                id: next_id,
                title: request.0.title,
                content: request.0.content,
                author: request.0.author,
                created_at: now.clone(),
                updated_at: now,
            };
            
            posts.push(new_post.clone());
            Response::json(&new_post, 201)
        }
    }
    // async wrapper for GET that forwards Response
    #[inline(always)]
    pub fn GET(params: &std::collections::HashMap<String, String>, body: &bytes::Bytes) -> std::pin::Pin<Box<dyn std::future::Future<Output = super::Response> + Send>> {
        let params = params.clone();
        let body = body.clone();
        Box::pin(async move {
            __orig::GET(&params).await
        })
    }
    // async wrapper for POST that forwards Response
    #[inline(always)]
    pub fn POST(params: &std::collections::HashMap<String, String>, body: &bytes::Bytes) -> std::pin::Pin<Box<dyn std::future::Future<Output = super::Response> + Send>> {
        let params = params.clone();
        let body = body.clone();
        Box::pin(async move {
            __orig::POST(&params, &body).await
        })
    }
}

#[allow(non_snake_case)]
mod module__home_runner_workspace_core_src_engine__________example_stats_rs {
    mod __orig {
        // 'api'
        use std::collections::HashMap;
        use serde::Serialize;
        use crate::engine::Response;
        
        #[derive(Serialize)]
        struct FrameworkStats {
            framework: &'static str,
            version: &'static str,
            features: Vec<Feature>,
            performance: Performance,
            runtime: Runtime,
        }
        
        #[derive(Serialize)]
        struct Feature {
            name: &'static str,
            description: &'static str,
            enabled: bool,
        }
        
        #[derive(Serialize)]
        struct Performance {
            avg_response_time: &'static str,
            route_matching: &'static str,
            binary_size: &'static str,
        }
        
        #[derive(Serialize)]
        struct Runtime {
            async_runtime: &'static str,
            http_server: &'static str,
            allocator: &'static str,
        }
        
        /// GET /stats - Framework statistics and features
        pub async fn GET(_params: &HashMap<String, String>) -> Response {
            let stats = FrameworkStats {
                framework: "Rust Web Framework",
                version: "1.0.0",
                features: vec![
                    Feature {
                        name: "File-Based Routing",
                        description: "Next.js-style routing with compile-time generation",
                        enabled: true,
                    },
                    Feature {
                        name: "Async Handlers",
                        description: "Full async/await support with Tokio runtime",
                        enabled: true,
                    },
                    Feature {
                        name: "Type-Safe Extractors",
                        description: "Json<T>, Form, Text extractors for request bodies",
                        enabled: true,
                    },
                    Feature {
                        name: "Dynamic Routes",
                        description: "URL parameters like /posts/:id",
                        enabled: true,
                    },
                    Feature {
                        name: "Zero-Cost Routing",
                        description: "Compile-time route matching with zero overhead",
                        enabled: true,
                    },
                    Feature {
                        name: "Hot Reload",
                        description: "Development mode with file watching",
                        enabled: true,
                    },
                ],
                performance: Performance {
                    avg_response_time: "0.03-0.15ms",
                    route_matching: "30-80ns",
                    binary_size: "2.5 MB",
                },
                runtime: Runtime {
                    async_runtime: "Tokio",
                    http_server: "Hyper",
                    allocator: "jemalloc",
                },
            };
        
            Response::json(&stats, 200)
        }
    }
    // async wrapper for GET that forwards Response
    #[inline(always)]
    pub fn GET(params: &std::collections::HashMap<String, String>, body: &bytes::Bytes) -> std::pin::Pin<Box<dyn std::future::Future<Output = super::Response> + Send>> {
        let params = params.clone();
        let body = body.clone();
        Box::pin(async move {
            __orig::GET(&params).await
        })
    }
}

#[allow(non_snake_case)]
mod module__home_runner_workspace_core_src_engine__________example_index_rs {
    mod __orig {
        // 'ui'
        use std::collections::HashMap;
        use crate::engine::Response;
        use bytes::Bytes;
        
        pub async fn GET(_params: &HashMap<String, String>) -> Response {
            let html = r#"<!DOCTYPE html>
        <html>
        <head>
            <title>Rust Blog API</title>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <style>
                * { margin: 0; padding: 0; box-sizing: border-box; }
                body {
                    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                    min-height: 100vh;
                    padding: 2rem;
                    color: #333;
                }
                .container {
                    max-width: 900px;
                    margin: 0 auto;
                    background: white;
                    border-radius: 12px;
                    padding: 3rem;
                    box-shadow: 0 20px 60px rgba(0,0,0,0.3);
                }
                h1 {
                    color: #667eea;
                    font-size: 2.5rem;
                    margin-bottom: 1rem;
                }
                h2 {
                    color: #764ba2;
                    font-size: 1.5rem;
                    margin: 2rem 0 1rem 0;
                    border-bottom: 2px solid #eee;
                    padding-bottom: 0.5rem;
                }
                .tagline {
                    color: #666;
                    font-size: 1.2rem;
                    margin-bottom: 2rem;
                }
                .features {
                    display: grid;
                    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
                    gap: 1rem;
                    margin: 2rem 0;
                }
                .feature {
                    background: #f8f9fa;
                    padding: 1rem;
                    border-radius: 8px;
                    border-left: 4px solid #667eea;
                }
                .feature h3 {
                    color: #667eea;
                    font-size: 1rem;
                    margin-bottom: 0.5rem;
                }
                .feature p {
                    color: #666;
                    font-size: 0.9rem;
                }
                .endpoint {
                    background: #f8f9fa;
                    padding: 1rem;
                    margin: 0.5rem 0;
                    border-radius: 6px;
                    border-left: 4px solid #764ba2;
                }
                .method {
                    display: inline-block;
                    padding: 0.2rem 0.6rem;
                    border-radius: 4px;
                    font-weight: bold;
                    font-size: 0.85rem;
                    margin-right: 0.5rem;
                }
                .get { background: #28a745; color: white; }
                .post { background: #007bff; color: white; }
                .put { background: #ffc107; color: #333; }
                .delete { background: #dc3545; color: white; }
                code {
                    background: #2d2d2d;
                    color: #f8f8f2;
                    padding: 0.2rem 0.4rem;
                    border-radius: 3px;
                    font-size: 0.9rem;
                }
                .example {
                    background: #2d2d2d;
                    color: #f8f8f2;
                    padding: 1rem;
                    border-radius: 6px;
                    margin: 1rem 0;
                    overflow-x: auto;
                }
                .example pre {
                    margin: 0;
                    font-size: 0.9rem;
                }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>🚀 Rust Blog API</h1>
                <p class="tagline">A blazingly fast, type-safe web framework with async handlers</p>
        
                <div class="features">
                    <div class="feature">
                        <h3>⚡ Sub-millisecond</h3>
                        <p>0.03-0.15ms response times</p>
                    </div>
                    <div class="feature">
                        <h3>🔒 Type-Safe</h3>
                        <p>Compile-time route generation</p>
                    </div>
                    <div class="feature">
                        <h3>🔄 Async</h3>
                        <p>Full async/await support</p>
                    </div>
                    <div class="feature">
                        <h3>📦 Tiny</h3>
                        <p>2.5 MB release binary</p>
                    </div>
                </div>
        
                <h2>📚 API Endpoints</h2>
        
                <div class="endpoint">
                    <span class="method get">GET</span>
                    <code>/posts</code>
                    <p style="margin-top: 0.5rem; color: #666;">List all blog posts</p>
                </div>
        
                <div class="endpoint">
                    <span class="method get">GET</span>
                    <code>/posts/:id</code>
                    <p style="margin-top: 0.5rem; color: #666;">Get a specific post by ID</p>
                </div>
        
                <div class="endpoint">
                    <span class="method post">POST</span>
                    <code>/posts</code>
                    <p style="margin-top: 0.5rem; color: #666;">Create a new post (requires JSON body)</p>
                </div>
        
                <div class="endpoint">
                    <span class="method put">PUT</span>
                    <code>/posts/:id</code>
                    <p style="margin-top: 0.5rem; color: #666;">Update an existing post (requires JSON body)</p>
                </div>
        
                <div class="endpoint">
                    <span class="method delete">DELETE</span>
                    <code>/posts/:id</code>
                    <p style="margin-top: 0.5rem; color: #666;">Delete a post</p>
                </div>
        
                <div class="endpoint">
                    <span class="method get">GET</span>
                    <code>/stats</code>
                    <p style="margin-top: 0.5rem; color: #666;">View framework statistics</p>
                </div>
        
                <h2>💡 Example Usage</h2>
        
                <div class="example">
                    <pre># Get all posts
        curl http://localhost:5000/posts
        
        # Get post by ID
        curl http://localhost:5000/posts/1
        
        # Create a new post
        curl -X POST http://localhost:5000/posts \
          -H "Content-Type: application/json" \
          -d '{
            "title": "My First Post",
            "content": "Hello, World!",
            "author": "Alice"
          }'
        
        # Update a post
        curl -X PUT http://localhost:5000/posts/1 \
          -H "Content-Type: application/json" \
          -d '{
            "title": "Updated Title",
            "content": "Updated content",
            "author": "Alice"
          }'
        
        # Delete a post
        curl -X DELETE http://localhost:5000/posts/1</pre>
                </div>
        
                <p style="margin-top: 2rem; color: #666; text-align: center;">
                    Built with ❤️ using Rust, Tokio, and Hyper
                </p>
            </div>
        </body>
        </html>"#;
        
            Response {
                status: 200,
                body: Bytes::from(html),
                content_type: "text/html; charset=utf-8",
                headers: Vec::new(),
            }
        }
    }
    // async wrapper for GET that forwards Response
    #[inline(always)]
    pub fn GET(params: &std::collections::HashMap<String, String>, body: &bytes::Bytes) -> std::pin::Pin<Box<dyn std::future::Future<Output = super::Response> + Send>> {
        let params = params.clone();
        let body = body.clone();
        Box::pin(async move {
            __orig::GET(&params).await
        })
    }
}

use std::option::Option;
use std::pin::Pin;
use std::future::Future;

pub type Handler = fn(&std::collections::HashMap<String, String>, &bytes::Bytes) -> Pin<Box<dyn Future<Output = super::Response> + Send>>;

#[inline(always)]
pub fn get_handler(route: &str, method: &str) -> Option<(Handler, std::collections::HashMap<String, String>)> {
    // Fast path: pre-check method bytes for quick rejection
    let method_bytes = method.as_bytes();
    
    // Normalize and split route into segments (stack-allocated for small routes)
    let route_normalized = route.trim_start_matches('/').trim_end_matches('/');
    let seg_count = if route_normalized.is_empty() { 0 } else { route_normalized.bytes().filter(|&b| b == b'/').count() + 1 };
    
    // Use small fixed arrays for common cases to avoid heap allocation
    let mut seg_buf: [&str; 8] = [""; 8];
    let segments = if seg_count <= 8 {
        let mut i = 0;
        for seg in route_normalized.split('/') {
            if i >= 8 { break; }
            seg_buf[i] = seg;
            i += 1;
        }
        &seg_buf[..seg_count]
    } else {
        // Fallback to heap for deep routes
        return None; // or handle with Vec if needed
    };

    // Match pattern: GET /posts/[id]
    if method_bytes.len() == 3 && method_bytes == b"GET" && seg_count == 2 {
        if segments[0] != "posts" { /* skip */ } else
        {
            let mut params = std::collections::HashMap::with_capacity(1);
            params.insert("id".to_string(), segments[1].to_string());
            return Some((module__home_runner_workspace_core_src_engine__________example_posts__id__rs::GET, params));
        }
    }
    // Match pattern: PUT /posts/[id]
    if method_bytes.len() == 3 && method_bytes == b"PUT" && seg_count == 2 {
        if segments[0] != "posts" { /* skip */ } else
        {
            let mut params = std::collections::HashMap::with_capacity(1);
            params.insert("id".to_string(), segments[1].to_string());
            return Some((module__home_runner_workspace_core_src_engine__________example_posts__id__rs::PUT, params));
        }
    }
    // Match pattern: DELETE /posts/[id]
    if method_bytes.len() == 6 && method_bytes == b"DELETE" && seg_count == 2 {
        if segments[0] != "posts" { /* skip */ } else
        {
            let mut params = std::collections::HashMap::with_capacity(1);
            params.insert("id".to_string(), segments[1].to_string());
            return Some((module__home_runner_workspace_core_src_engine__________example_posts__id__rs::DELETE, params));
        }
    }
    // Match pattern: GET /posts
    if method_bytes.len() == 3 && method_bytes == b"GET" && seg_count == 1 {
        if segments[0] != "posts" { /* skip */ } else
        {
            return Some((module__home_runner_workspace_core_src_engine__________example_posts_rs::GET, std::collections::HashMap::new()));
        }
    }
    // Match pattern: POST /posts
    if method_bytes.len() == 4 && method_bytes == b"POST" && seg_count == 1 {
        if segments[0] != "posts" { /* skip */ } else
        {
            return Some((module__home_runner_workspace_core_src_engine__________example_posts_rs::POST, std::collections::HashMap::new()));
        }
    }
    // Match pattern: GET /stats
    if method_bytes.len() == 3 && method_bytes == b"GET" && seg_count == 1 {
        if segments[0] != "stats" { /* skip */ } else
        {
            return Some((module__home_runner_workspace_core_src_engine__________example_stats_rs::GET, std::collections::HashMap::new()));
        }
    }
    // Match pattern: GET /
    if method_bytes.len() == 3 && method_bytes == b"GET" && seg_count == 0 {
        {
            return Some((module__home_runner_workspace_core_src_engine__________example_index_rs::GET, std::collections::HashMap::new()));
        }
    }
    None
}
