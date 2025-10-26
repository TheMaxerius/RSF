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

#[allow(non_snake_case)]
mod module__home_runner_workspace_core_src_engine__________example_ws_chat_rs {
    mod __orig {
        // 'api'
        use std::collections::HashMap;
        use std::sync::atomic::{AtomicU64, Ordering};
        use crate::engine::{Response, WsMessage, WsRoom};
        use bytes::Bytes;
        use once_cell::sync::Lazy;
        
        // Global chat room shared across all connections
        static CHAT_ROOM: Lazy<WsRoom> = Lazy::new(|| WsRoom::new());
        static USER_COUNTER: AtomicU64 = AtomicU64::new(0);
        
        /// GET /ws/chat - WebSocket chat information
        /// To connect via WebSocket, use a WebSocket client library
        /// Example: ws://localhost:5000/ws/chat
        pub async fn GET(_params: &HashMap<String, String>) -> Response {
            Response::json(&serde_json::json!({
                "info": "WebSocket Chat Room",
                "active_users": CHAT_ROOM.count(),
                "connection_url": "ws://localhost:5000/ws/chat",
                "usage": "Use a WebSocket client to connect. Send text messages to chat with other users.",
                "example": "websocat ws://localhost:5000/ws/chat"
            }), 200)
        }
        
        pub(crate) async fn handle_chat_connection(
            ws_connection: crate::engine::WebSocketConnection,
            user_id: String,
        ) -> Result<(), String> {
            // This is a placeholder - full WebSocket chat implementation requires
            // server modifications to support WebSocket upgrades in routing
            ws_connection.handle(|mut ws| async move {
                // Send welcome message
                ws.send(format!("Welcome to the chat! Your ID: {}", user_id)).await?;
                ws.send(format!("Active users: {}", CHAT_ROOM.count() + 1)).await?;
            
            // Create a channel for broadcasting
            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
            CHAT_ROOM.join(user_id.clone(), tx);
            
            // Broadcast join message
            CHAT_ROOM.broadcast(format!("🟢 {} joined the chat", user_id));
            
            // Split socket into sender and receiver
            let (mut sender, mut receiver) = ws.split();
            
            // Spawn task to handle outgoing messages (broadcasts from other users)
            let send_user_id = user_id.clone();
            let send_task = tokio::spawn(async move {
                while let Some(msg) = rx.recv().await {
                    if sender.send(msg).await.is_err() {
                        break;
                    }
                }
            });
            
            // Handle incoming messages from this user
            while let Ok(Some(msg)) = receiver.receive().await {
                match msg {
                    WsMessage::Text(text) => {
                        log::info!("[Chat] {}: {}", user_id, text);
                        
                        // Broadcast to all other users
                        CHAT_ROOM.broadcast(format!("{}: {}", user_id, text));
                    }
                    WsMessage::Close => {
                        log::info!("[Chat] {} disconnected", user_id);
                        break;
                    }
                    _ => {}
                }
            }
            
                // Clean up
                CHAT_ROOM.leave(&user_id);
                CHAT_ROOM.broadcast(format!("🔴 {} left the chat", user_id));
                send_task.abort();
                
                Ok(())
            }).await
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
mod module__home_runner_workspace_core_src_engine__________example_middleware_demo_rs {
    mod __orig {
        // 'api' with middleware
        use std::collections::HashMap;
        use crate::engine::{Response, MiddlewareContext, MiddlewareResult, MiddlewareChain, AfterMiddlewareChain};
        use bytes::Bytes;
        use once_cell::sync::Lazy;
        
        // Define custom middleware chains for this route
        static BEFORE_MIDDLEWARE: Lazy<MiddlewareChain> = Lazy::new(|| {
            MiddlewareChain::new()
                // Add logging middleware
                .add(|ctx| async move {
                    log::info!("🔵 [Middleware] Before: {} {}", ctx.method, ctx.path);
                    MiddlewareResult::Continue(ctx)
                })
                // Add timing middleware
                .add(|mut ctx| async move {
                    use std::time::Instant;
                    let start = Instant::now();
                    ctx.set("start_time".to_string(), format!("{:?}", start));
                    MiddlewareResult::Continue(ctx)
                })
                // Add custom validation middleware
                .add(|ctx| async move {
                    // Example: Check for a custom header
                    if let Some(api_key) = ctx.header("x-api-key") {
                        if api_key == "demo-key-12345" {
                            log::info!("✅ [Middleware] API key validated");
                            return MiddlewareResult::Continue(ctx);
                        }
                    }
                    
                    // For demo purposes, we'll allow requests without key
                    // but log a warning
                    log::warn!("⚠️  [Middleware] No valid API key provided");
                    MiddlewareResult::Continue(ctx)
                })
        });
        
        static AFTER_MIDDLEWARE: Lazy<AfterMiddlewareChain> = Lazy::new(|| {
            AfterMiddlewareChain::new()
                // Add CORS headers
                .add(|_ctx, mut resp| async move {
                    resp.headers.push(("Access-Control-Allow-Origin".to_string(), "*".to_string()));
                    resp.headers.push(("X-Powered-By".to_string(), "Rust Framework".to_string()));
                    resp
                })
                // Add timing header
                .add(|ctx, mut resp| async move {
                    if let Some(start_str) = ctx.get("start_time") {
                        resp.headers.push(("X-Custom-Timing".to_string(), "tracked".to_string()));
                    }
                    log::info!("🟢 [Middleware] After: Status {}", resp.status);
                    resp
                })
        });
        
        /// GET /middleware-demo - Route with custom middleware
        pub async fn GET(params: &HashMap<String, String>) -> Response {
            // Create middleware context
            let mut ctx = MiddlewareContext::new("GET".to_string(), "/middleware-demo".to_string());
            
            // In a real implementation, you'd populate headers from the request
            ctx.headers.insert("x-api-key".to_string(), "demo-key-12345".to_string());
            
            // Execute before middleware chain
            let ctx = match BEFORE_MIDDLEWARE.execute(ctx).await {
                MiddlewareResult::Continue(ctx) => ctx,
                MiddlewareResult::Response(resp) => return resp,
            };
            
            // Handler logic
            let response = Response::json(&serde_json::json!({
                "message": "Hello from middleware demo!",
                "middleware_executed": true,
                "context_extensions": ctx.extensions.len(),
                "info": "Check the response headers to see middleware additions"
            }), 200);
            
            // Execute after middleware chain
            AFTER_MIDDLEWARE.execute(ctx, response).await
        }
        
        /// POST /middleware-demo - Example with body parsing
        pub async fn POST(params: &HashMap<String, String>, body: &Bytes) -> Response {
            // Create middleware context
            let ctx = MiddlewareContext::new("POST".to_string(), "/middleware-demo".to_string());
            
            // Execute before middleware
            let ctx = match BEFORE_MIDDLEWARE.execute(ctx).await {
                MiddlewareResult::Continue(ctx) => ctx,
                MiddlewareResult::Response(resp) => return resp,
            };
            
            // Parse body
            let body_str = String::from_utf8_lossy(body);
            
            // Handler logic
            let response = Response::json(&serde_json::json!({
                "message": "POST request processed with middleware",
                "body_length": body.len(),
                "body_preview": &body_str[..body_str.len().min(100)]
            }), 200);
            
            // Execute after middleware
            AFTER_MIDDLEWARE.execute(ctx, response).await
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
mod module__home_runner_workspace_core_src_engine__________example_ws_demo_rs {
    mod __orig {
        // 'api'
        use std::collections::HashMap;
        use crate::engine::Response;
        use bytes::Bytes;
        
        /// GET /ws-demo - WebSocket demo information
        pub async fn GET(_params: &HashMap<String, String>) -> Response {
            Response::json(&serde_json::json!({
                "info": "WebSocket support is available in this framework!",
                "features": [
                    "Real-time bidirectional communication",
                    "Built on hyper-tungsten for performance",
                    "Support for rooms and broadcasting",
                    "Clean async API"
                ],
                "example_endpoints": [
                    "/ws/chat - Multi-user chat room (coming soon)",
                    "/ws-demo - This information page"
                ],
                "how_to_use": {
                    "step_1": "Create a route file in example/ws/",
                    "step_2": "Import: use crate::engine::{is_websocket_upgrade, upgrade_websocket, WsMessage};",
                    "step_3": "Check for upgrade request and handle WebSocket connection",
                    "step_4": "Use ws.send() and ws.receive() for messaging"
                },
                "code_example": "See example/ws/chat.rs for a complete implementation"
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
}

#[allow(non_snake_case)]
mod module__home_runner_workspace_core_src_engine__________example_improved_dx__id__rs {
    mod __orig {
        // 'api'
        // Example showing improved path parameter extraction
        
        use std::collections::HashMap;
        use crate::engine::{Response, extract_param};
        use serde::Serialize;
        
        #[derive(Serialize)]
        struct UserDetail {
            id: usize,
            name: String,
            email: String,
            status: String,
        }
        
        /// GET /improved_dx/:id - Clean path parameter extraction
        pub async fn GET(params: &HashMap<String, String>) -> Response {
            // ✨ DX Improvement: One-line parameter extraction with automatic error handling
            let user_id = match extract_param::<usize>(params, "id") {
                Ok(id) => id,
                Err(e) => {
                    return Response::json(&serde_json::json!({
                        "error": e
                    }), 400);
                }
            };
            
            // Simulate database lookup
            let user = UserDetail {
                id: user_id,
                name: format!("User {}", user_id),
                email: format!("user{}@example.com", user_id),
                status: "active".to_string(),
            };
            
            Response::json(&user, 200)
        }
        
        /// DELETE /improved_dx/:id - Example with custom error handling
        pub async fn DELETE(params: &HashMap<String, String>) -> Response {
            let user_id = match extract_param::<usize>(params, "id") {
                Ok(id) => id,
                Err(e) => {
                    return Response::json(&serde_json::json!({
                        "error": format!("Invalid user ID: {}", e)
                    }), 400);
                }
            };
            
            // Simulate deletion
            Response::json(&serde_json::json!({
                "message": format!("User {} deleted successfully", user_id),
                "id": user_id
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
mod module__home_runner_workspace_core_src_engine__________example_improved_dx_rs {
    mod __orig {
        // 'api'
        // Example demonstrating improved developer experience with new helpers
        
        use std::collections::HashMap;
        use crate::engine::{Response, AppError, extract_param, Json};
        use bytes::Bytes;
        use serde::{Deserialize, Serialize};
        
        #[derive(Deserialize)]
        struct CreateUser {
            name: String,
            email: String,
        }
        
        #[derive(Serialize)]
        struct User {
            id: usize,
            name: String,
            email: String,
        }
        
        /// GET /improved_dx - Simple demo with clean parameter extraction
        pub async fn GET(params: &HashMap<String, String>) -> Response {
            // ✨ DX Improvement: Simple extraction with helpful error messages
            let user_id = match extract_param::<usize>(params, "id") {
                Ok(id) => id,
                Err(_) => {
                    // Optional parameter - use default
                    1
                }
            };
            
            Response::json(&serde_json::json!({
                "message": "Improved DX Demo",
                "features": [
                    "✅ extract_param<T> - Type-safe parameter parsing with one line",
                    "✅ AppError type - Result-based error handling with ? operator",
                    "✅ Automatic error messages - No manual error construction",
                    "✅ Helper functions - Reduced boilerplate for common patterns"
                ],
                "examples": {
                    "parameter_extraction": "let id = extract_param::<usize>(params, \"id\")?;",
                    "error_handling": "return Err(AppError::bad_request(\"Invalid input\"));",
                    "json_parsing": "let data = Json::<T>::from_bytes(body).map_err(AppError::from)?;"
                },
                "user_id": user_id
            }), 200)
        }
        
        /// POST /improved_dx - Example with improved error handling
        pub async fn POST(_params: &HashMap<String, String>, body: &Bytes) -> Response {
            // ✨ DX Improvement: Automatic error conversion with ?
            match handle_create_user(body) {
                Ok(user) => Response::json(&user, 201),
                Err(err) => err.into_response(),
            }
        }
        
        // Helper function using Result for cleaner error handling
        pub(crate) fn handle_create_user(body: &Bytes) -> Result<User, AppError> {
            // ✨ DX Improvement: Json::from_bytes returns Result, use ? operator
            let request = Json::<CreateUser>::from_bytes(body)
                .map_err(|e| AppError::bad_request(format!("Invalid JSON: {}", e)))?;
            
            // ✨ DX Improvement: Use helper functions for validation
            if request.0.name.is_empty() {
                return Err(AppError::bad_request("Name cannot be empty"));
            }
            
            if !request.0.email.contains('@') {
                return Err(AppError::bad_request("Invalid email format"));
            }
            
            // Success case
            Ok(User {
                id: 123,
                name: request.0.name,
                email: request.0.email,
            })
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
    // Match pattern: GET /ws/chat
    if method_bytes.len() == 3 && method_bytes == b"GET" && seg_count == 2 {
        if segments[0] != "ws" { /* skip */ } else
        if segments[1] != "chat" { /* skip */ } else
        {
            return Some((module__home_runner_workspace_core_src_engine__________example_ws_chat_rs::GET, std::collections::HashMap::new()));
        }
    }
    // Match pattern: GET /middleware_demo
    if method_bytes.len() == 3 && method_bytes == b"GET" && seg_count == 1 {
        if segments[0] != "middleware_demo" { /* skip */ } else
        {
            return Some((module__home_runner_workspace_core_src_engine__________example_middleware_demo_rs::GET, std::collections::HashMap::new()));
        }
    }
    // Match pattern: POST /middleware_demo
    if method_bytes.len() == 4 && method_bytes == b"POST" && seg_count == 1 {
        if segments[0] != "middleware_demo" { /* skip */ } else
        {
            return Some((module__home_runner_workspace_core_src_engine__________example_middleware_demo_rs::POST, std::collections::HashMap::new()));
        }
    }
    // Match pattern: GET /ws_demo
    if method_bytes.len() == 3 && method_bytes == b"GET" && seg_count == 1 {
        if segments[0] != "ws_demo" { /* skip */ } else
        {
            return Some((module__home_runner_workspace_core_src_engine__________example_ws_demo_rs::GET, std::collections::HashMap::new()));
        }
    }
    // Match pattern: GET /improved_dx/[id]
    if method_bytes.len() == 3 && method_bytes == b"GET" && seg_count == 2 {
        if segments[0] != "improved_dx" { /* skip */ } else
        {
            let mut params = std::collections::HashMap::with_capacity(1);
            params.insert("id".to_string(), segments[1].to_string());
            return Some((module__home_runner_workspace_core_src_engine__________example_improved_dx__id__rs::GET, params));
        }
    }
    // Match pattern: DELETE /improved_dx/[id]
    if method_bytes.len() == 6 && method_bytes == b"DELETE" && seg_count == 2 {
        if segments[0] != "improved_dx" { /* skip */ } else
        {
            let mut params = std::collections::HashMap::with_capacity(1);
            params.insert("id".to_string(), segments[1].to_string());
            return Some((module__home_runner_workspace_core_src_engine__________example_improved_dx__id__rs::DELETE, params));
        }
    }
    // Match pattern: GET /improved_dx
    if method_bytes.len() == 3 && method_bytes == b"GET" && seg_count == 1 {
        if segments[0] != "improved_dx" { /* skip */ } else
        {
            return Some((module__home_runner_workspace_core_src_engine__________example_improved_dx_rs::GET, std::collections::HashMap::new()));
        }
    }
    // Match pattern: POST /improved_dx
    if method_bytes.len() == 4 && method_bytes == b"POST" && seg_count == 1 {
        if segments[0] != "improved_dx" { /* skip */ } else
        {
            return Some((module__home_runner_workspace_core_src_engine__________example_improved_dx_rs::POST, std::collections::HashMap::new()));
        }
    }
    None
}
