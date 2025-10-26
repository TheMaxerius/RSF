# ⚡ Rust Web Framework - Ultra-Fast File-Based Routing

A blazingly fast, zero-overhead web framework for Rust with Next.js-style file-based routing, async handlers, WebSocket support, middleware system, and sub-3MB binaries.

---

## 📋 Table of Contents

- [Features](#-features)
- [Quick Start](#-quick-start)
- [Blog API Demo](#-blog-api-demo)
- [WebSocket Support](#-websocket-support)
- [Middleware System](#-middleware-system)
- [Developer Experience Features](#-developer-experience-features)
- [How It Works](#-how-it-works)
- [Performance](#-performance)
- [Architecture](#-architecture)
- [Documentation](#-documentation)

---

## 🚀 Features

### Performance First

- **2.5 MB release binary** with full LTO optimization
- **0.03-0.15ms response times** (sub-millisecond!)
- **30-80ns route matching** (compile-time code generation)
- **Zero heap allocations** for route matching (stack-based)
- **jemalloc allocator** for superior memory performance
- **AHashMap** for 2x faster hashing than std HashMap
- **Full async/await** support with Tokio runtime

### Developer Experience

- **File-based routing** like Next.js: `users/[id].rs` → `/users/:id`
- **Async handlers** with Pin<Box<dyn Future>> for concurrency
- **Type-safe extractors** (`Json<T>`, `Form`, `Text`, `RawBody`)
- **DX improvements** - `extract_param<T>` helper, `AppError` type, Result-based error handling
- **WebSocket support** with hyper-tungstenite integration and clean async API
- **Middleware system** with before/after hooks and helper functions
- **Hot reload** during development (watches .rs files)
- **Colored terminal output** with request logging
- **Zero configuration** - just create route files

### Routing Capabilities

- ✅ Static routes: `/api`, `/health`, `/users`
- ✅ Dynamic parameters: `/users/[id]`, `/posts/[slug]`
- ✅ Nested dynamic routes: `/posts/[id]/comments/[commentId]`
- ✅ Multiple HTTP methods: GET, POST, PUT, DELETE, PATCH, OPTIONS, HEAD
- ✅ Request body parsing with type-safe extractors
- ✅ URL decoding built-in
- ✅ Unlimited nesting depth
- ✅ WebSocket connections with real-time bidirectional communication
- ✅ Middleware with before/after hooks for auth, logging, CORS, etc.
- ✅ DX helpers: `extract_param<T>` for one-line parameter parsing
- ✅ `AppError` type for Result-based error handling with `?` operator

## 📦 Quick Start

### Installation

```bash
git clone <repo>
cd core
cargo build --release
```

### Create Your First Route

Create `example/hello.rs`:

```rust
// 'api'
use std::collections::HashMap;
use crate::engine::Response;

pub async fn GET(_params: &HashMap<String, String>) -> Response {
    Response::json(&serde_json::json!({
        "message": "Hello, World!",
        "framework": "Rust Web Framework"
    }), 200)
}
```

### Run the Server

```bash
cargo run
# Server listening on http://0.0.0.0:5000
```

### Test it

```bash
curl http://localhost:5000/hello
# {"message":"Hello, World!","framework":"Rust Web Framework"}
```

## 📚 Blog API Demo

The framework includes a full-featured blog API demo showcasing all capabilities.

### Live Demo Routes

1. **`/`** - Beautiful landing page with API documentation
2. **`/posts`** - List all posts (GET) and create posts (POST)
3. **`/posts/:id`** - Get, update, or delete individual posts
4. **`/stats`** - Framework features and performance metrics

### Example: List Posts (`example/posts.rs`)

```rust
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

// Global in-memory database
pub static POSTS: Lazy<Arc<Mutex<Vec<Post>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(vec![/* initial posts */]))
});

/// GET /posts - List all posts
pub async fn GET(_params: &HashMap<String, String>) -> Response {
    let posts = POSTS.lock().unwrap();
    Response::json(&*posts, 200)
}

/// POST /posts - Create a new post
pub async fn POST(_params: &HashMap<String, String>, body: &Bytes) -> Response {
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
    
    let new_post = Post {
        id: next_id,
        title: request.0.title,
        content: request.0.content,
        author: request.0.author,
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    };
    
    posts.push(new_post.clone());
    Response::json(&new_post, 201)
}
```

### Example: Dynamic Route with Improved DX (`example/improved_dx/[id].rs`)

```rust
// 'api'
use std::collections::HashMap;
use crate::engine::{Response, extract_param};

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
    
    // Your handler logic here
    let user = UserDetail {
        id: user_id,
        name: format!("User {}", user_id),
        email: format!("user{}@example.com", user_id),
        status: "active".to_string(),
    };
    
    Response::json(&user, 200)
}
```

### Example: Result-Based Error Handling (`example/improved_dx.rs`)

```rust
// 'api'
use crate::engine::{Response, AppError, Json};
use bytes::Bytes;

pub async fn POST(_params: &HashMap<String, String>, body: &Bytes) -> Response {
    // ✨ DX Improvement: Use Result and ? operator for cleaner error handling
    match handle_create_user(body) {
        Ok(user) => Response::json(&user, 201),
        Err(err) => err.into_response(),
    }
}

fn handle_create_user(body: &Bytes) -> Result<User, AppError> {
    // Use ? operator for automatic error propagation
    let request = Json::<CreateUser>::from_bytes(body)
        .map_err(|e| AppError::bad_request(format!("Invalid JSON: {}", e)))?;
    
    // Validation with semantic error types
    if request.0.name.is_empty() {
        return Err(AppError::bad_request("Name cannot be empty"));
    }
    
    if !request.0.email.contains('@') {
        return Err(AppError::bad_request("Invalid email format"));
    }
    
    Ok(User {
        id: 123,
        name: request.0.name,
        email: request.0.email,
    })
}
```

### Test the Blog API

```bash
# List all posts
curl http://localhost:5000/posts

# Create a post
curl -X POST http://localhost:5000/posts \
  -H "Content-Type: application/json" \
  -d '{"title":"My First Post","content":"Hello from Rust!","author":"Alice"}'

# Get post by ID
curl http://localhost:5000/posts/1

# Update a post
curl -X PUT http://localhost:5000/posts/1 \
  -H "Content-Type: application/json" \
  -d '{"title":"Updated Title","content":"New content","author":"Alice"}'

# Delete a post
curl -X DELETE http://localhost:5000/posts/1

# View framework stats
curl http://localhost:5000/stats

# Test DX improvements
curl http://localhost:5000/improved_dx
curl http://localhost:5000/improved_dx/123
curl -X POST http://localhost:5000/improved_dx \
  -H "Content-Type: application/json" \
  -d '{"name":"Alice","email":"alice@example.com"}'
```

## 🔌 WebSocket Support

The framework includes built-in WebSocket support for real-time bidirectional communication.

### Basic WebSocket Handler

Create `example/ws/echo.rs`:

```rust
// 'api'
use std::collections::HashMap;
use crate::engine::{Response, is_websocket_upgrade, upgrade_websocket, WsMessage};
use hyper::{Request, Body};

pub async fn GET(_params: &HashMap<String, String>, req: &mut Request<Body>) -> Response {
    if !is_websocket_upgrade(req) {
        return Response::json(&serde_json::json!({
            "error": "WebSocket upgrade required"
        }), 400);
    }
    
    let (response, ws_connection) = match upgrade_websocket(req) {
        Ok(result) => result,
        Err(e) => return Response::json(&serde_json::json!({"error": e}), 500),
    };
    
    tokio::spawn(async move {
        ws_connection.handle(|mut ws| async move {
            ws.send("Connected!").await?;
            
            while let Ok(Some(msg)) = ws.receive().await {
                match msg {
                    WsMessage::Text(text) => {
                        ws.send(format!("Echo: {}", text)).await?;
                    }
                    WsMessage::Close => break,
                    _ => {}
                }
            }
            Ok(())
        }).await
    });
    
    Response {
        body: hyper::body::to_bytes(response.into_body()).await.unwrap_or_default(),
        status: response.status().as_u16(),
        content_type: "text/plain",
        headers: vec![],
    }
}
```

### WebSocket with Broadcasting (Chat Room)

```rust
// 'api'
use once_cell::sync::Lazy;
use crate::engine::WsRoom;

static CHAT_ROOM: Lazy<WsRoom> = Lazy::new(|| WsRoom::new());

async fn handle_chat(ws_connection: WebSocketConnection, user_id: String) -> Result<(), String> {
    ws_connection.handle(|ws| async move {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        CHAT_ROOM.join(user_id.clone(), tx);
        
        CHAT_ROOM.broadcast(format!("{} joined", user_id));
        
        let (mut sender, mut receiver) = ws.split();
        
        // Send broadcasts to this user
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if sender.send(msg).await.is_err() { break; }
            }
        });
        
        // Receive messages from this user
        while let Ok(Some(msg)) = receiver.receive().await {
            if let WsMessage::Text(text) = msg {
                CHAT_ROOM.broadcast(format!("{}: {}", user_id, text));
            }
        }
        
        CHAT_ROOM.leave(&user_id);
        Ok(())
    }).await
}
```

### Test WebSocket

```bash
# Using websocat (install: cargo install websocat)
websocat ws://localhost:5000/ws/echo

# Or test the example endpoints
curl http://localhost:5000/ws_demo
curl http://localhost:5000/ws/chat
```

**📘 See [`WEBSOCKET_GUIDE.md`](WEBSOCKET_GUIDE.md) for complete WebSocket documentation**

## ⚙️ Middleware System

Add custom logic before/after request handling with middleware chains.

### Basic Middleware

```rust
// 'api'
use crate::engine::{MiddlewareContext, MiddlewareResult, MiddlewareChain, AfterMiddlewareChain};
use once_cell::sync::Lazy;

static BEFORE_MIDDLEWARE: Lazy<MiddlewareChain> = Lazy::new(|| {
    MiddlewareChain::new()
        .add(|ctx| async move {
            log::info!("Request: {} {}", ctx.method, ctx.path);
            MiddlewareResult::Continue(ctx)
        })
});

static AFTER_MIDDLEWARE: Lazy<AfterMiddlewareChain> = Lazy::new(|| {
    AfterMiddlewareChain::new()
        .add(|_ctx, mut resp| async move {
            resp.headers.push(("X-Custom-Header".to_string(), "value".to_string()));
            resp
        })
});

pub async fn GET(params: &HashMap<String, String>) -> Response {
    let ctx = MiddlewareContext::new("GET".to_string(), "/endpoint".to_string());
    
    // Execute before middleware
    let ctx = match BEFORE_MIDDLEWARE.execute(ctx).await {
        MiddlewareResult::Continue(ctx) => ctx,
        MiddlewareResult::Response(resp) => return resp,
    };
    
    // Handler logic
    let response = Response::json(&serde_json::json!({"message": "Hello!"}), 200);
    
    // Execute after middleware
    AFTER_MIDDLEWARE.execute(ctx, response).await
}
```

### Common Middleware Examples

**Authentication:**
```rust
.add(|ctx| async move {
    if let Some(auth) = ctx.header("authorization") {
        if validate_token(auth) {
            return MiddlewareResult::Continue(ctx);
        }
    }
    MiddlewareResult::Response(Response::json(&serde_json::json!({
        "error": "Unauthorized"
    }), 401))
})
```

**CORS:**
```rust
.add(|_ctx, mut resp| async move {
    resp.headers.push(("Access-Control-Allow-Origin".to_string(), "*".to_string()));
    resp
})
```

**Timing:**
```rust
// Before
.add(|mut ctx| async move {
    ctx.set("start_time".to_string(), format!("{:?}", Instant::now()));
    MiddlewareResult::Continue(ctx)
})

// After
.add(|ctx, mut resp| async move {
    if let Some(_start) = ctx.get("start_time") {
        resp.headers.push(("X-Response-Time".to_string(), "tracked".to_string()));
    }
    resp
})
```

### Test Middleware

```bash
# Test the middleware demo endpoint
curl http://localhost:5000/middleware_demo
```

**📘 See [`MIDDLEWARE_GUIDE.md`](MIDDLEWARE_GUIDE.md) for complete middleware documentation**

## 🎯 Developer Experience Features

### 1. One-Line Parameter Extraction

Stop writing nested match statements for path parameters:

```rust
// Before: Manual parsing (10+ lines)
let id: usize = match params.get("id") {
    Some(id_str) => match id_str.parse() {
        Ok(n) => n,
        Err(_) => return Response::json(&error, 400),
    },
    None => return Response::json(&error, 400),
};

// After: One line with extract_param<T>
use crate::engine::extract_param;
let id = extract_param::<usize>(params, "id")?;
```

### 2. Result-Based Error Handling

Use the `?` operator for cleaner error handling:

```rust
use crate::engine::AppError;

fn validate_user(data: &CreateUser) -> Result<User, AppError> {
    if data.name.is_empty() {
        return Err(AppError::bad_request("Name required"));
    }
    
    if !data.email.contains('@') {
        return Err(AppError::bad_request("Invalid email"));
    }
    
    Ok(user)
}

// Errors automatically convert to JSON responses
match validate_user(&data) {
    Ok(user) => Response::json(&user, 201),
    Err(err) => err.into_response(), // Automatic JSON error response
}
```

### 3. Helpful Error Messages

Framework provides clear, actionable error messages:
- `"Missing path parameter: id"`
- `"Invalid id parameter: not a number"`
- `"Invalid JSON: expected value at line 1 column 1"`

## ⚙️ How It Works

### Compile-Time Route Generation

The framework uses a `build.rs` script that:

1. **Scans `example/` directory** for `.rs` files at build time
2. **Generates async routing code** in `generated_routes.rs`
3. **Creates optimized matchers** for each route pattern
4. **Embeds everything** into the binary with zero overhead

### Async Handler Pipeline

All route handlers are async functions that return `Response`:

```rust
pub async fn GET(params: &HashMap<String, String>) -> Response {
    // Async operations supported
    Response::json(&data, 200)
}

pub async fn POST(params: &HashMap<String, String>, body: &Bytes) -> Response {
    // Body parameter for request parsing
    Response::json(&result, 201)
}
```

The build script generates async wrappers:

```rust
Box::pin(async move {
    let params = params.clone();
    let body = body.clone();
    route_module::POST(&params, &body).await
})
```

### Type-Safe Extractors

Parse request bodies with compile-time type checking and clean error handling:

```rust
use crate::engine::{Json, AppError};

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

pub async fn POST(_params: &HashMap<String, String>, body: &Bytes) -> Response {
    // Use Result and ? operator for cleaner error handling
    match handle_login(body) {
        Ok(response) => response,
        Err(err) => err.into_response(),
    }
}

fn handle_login(body: &Bytes) -> Result<Response, AppError> {
    let request = Json::<LoginRequest>::from_bytes(body)
        .map_err(|e| AppError::bad_request(format!("Invalid JSON: {}", e)))?;
    
    // Access typed data
    let username = &request.0.username;
    let password = &request.0.password;
    
    // Validation
    if username.is_empty() || password.is_empty() {
        return Err(AppError::bad_request("Username and password required"));
    }
    
    Ok(Response::json(&serde_json::json!({"success": true}), 200))
}
```

## 🏎️ Performance

### Real-World Benchmarks

| Endpoint          | Response Time |
| ----------------- | ------------- |
| GET /             | 0.02-0.03ms   |
| GET /posts        | 0.21ms        |
| POST /posts       | 0.12ms        |
| GET /posts/:id    | 0.12ms        |
| PUT /posts/:id    | 0.11ms        |
| DELETE /posts/:id | 0.08ms        |
| GET /stats        | 0.16ms        |

### Route Matching Performance

| Operation                     | Latency |
| ----------------------------- | ------- |
| Static route match            | ~30ns   |
| Dynamic route match (1 param) | ~62ns   |
| Nested route match (2 params) | ~80ns   |

### Binary Size

| Build   | Size       | Notes                 |
| ------- | ---------- | --------------------- |
| Debug   | 36 MB      | With debug symbols    |
| Release | **2.5 MB** | Full LTO, opt-level=3 |

### Memory Usage

- **Startup**: ~3 MB RSS
- **Per request**: Minimal heap allocations (async runtime overhead only)
- **Route matching**: 100% stack-allocated

## 🛠️ Architecture

```
.
├── core/              # Main framework core
│   ├── src/
│   │   ├── main.rs           # Entry point
│   │   ├── logger.rs         # Logging utilities
│   │   └── engine/           # Framework engine
│   │       ├── server.rs     # HTTP server
│   │       ├── handler.rs    # Request handling
│   │       ├── parser.rs     # Route parsing
│   │       ├── runtime.rs    # Runtime management
│   │       ├── auth.rs       # JWT & session auth
│   │       ├── middleware.rs # CORS, compression, etc.
│   │       ├── request.rs    # Request parsing
│   │       ├── extractors.rs # Type-safe body extractors
│   │       ├── error.rs      # AppError type for DX
│   │       ├── helpers.rs    # DX helper functions
│   │       ├── ws.rs         # WebSocket support
│   │       ├── mw.rs         # Middleware system
│   │       ├── static_files.rs # Static file serving
│   │       └── generated_routes.rs # Auto-generated routes
│   ├── build.rs      # Compile-time route generation
│   └── Cargo.toml    # Dependencies
├── example/          # Example route handlers
│   ├── index.rs
│   ├── posts.rs
│   ├── posts/[id].rs
│   ├── improved_dx.rs        # DX improvements demo
│   ├── improved_dx/[id].rs   # Parameter extraction demo
│   ├── middleware_demo.rs
│   ├── ws_demo.rs
│   ├── ws/
│   │   └── chat.rs
│   └── stats.rs
├── README.md         # This file
├── DX_IMPROVEMENTS.md # Developer experience guide
└── replit.md         # Project memory/architecture
```

## 📚 Documentation

Comprehensive guides are available for specific features:

- **[DX_IMPROVEMENTS.md](DX_IMPROVEMENTS.md)** - Complete guide to developer experience improvements
  - `extract_param<T>` helper for type-safe parameter parsing
  - `AppError` type for Result-based error handling
  - Migration guide from old patterns to new DX features
  - Code examples and best practices

- **Core Documentation** (in this README)
  - File-based routing system
  - WebSocket support
  - Middleware system
  - Performance benchmarks
  - Architecture overview

All features are demonstrated with working examples in the `example/` directory:
- `example/improved_dx.rs` - DX improvements showcase
- `example/improved_dx/[id].rs` - Parameter extraction demo
- `example/ws/chat.rs` - WebSocket chat room
- `example/middleware_demo.rs` - Middleware patterns
- `example/posts.rs` - Full CRUD API

## 🔧 Configuration

### Environment Variables

```bash
# Server port (default: 5000)
PORT=5000

# Server host (default: 0.0.0.0)
HOST=0.0.0.0

# Development mode (enables hot reload)
DEV=true

# Hot reload (default: auto-enabled in dev mode)
HOT_RELOAD=true

# Log level (default: info in prod, debug in dev)
RUST_LOG=debug
```

### Running in Different Modes

```bash
# Development mode with hot reload
cargo run

# Production mode
cargo run --release

# Custom port
PORT=3000 cargo run
```

## 🎯 Roadmap

### Completed ✅

- [x] File-based routing with dynamic parameters
- [x] Compile-time route generation
- [x] Full async/await support
- [x] Type-safe request body extractors (`Json<T>`, `Form`, `Text`, `RawBody`)
- [x] **DX improvements** - `extract_param<T>` and `AppError` type
- [x] Response helpers (Response::json)
- [x] Hyper HTTP server integration
- [x] Hot reload for development
- [x] Request/response logging
- [x] Environment configuration
- [x] jemalloc allocator for superior memory performance
- [x] Full LTO optimization (2.5MB binaries)
- [x] **WebSocket support** with hyper-tungstenite
- [x] **Middleware system** with before/after hooks
- [x] Result-based error handling

### Planned

- [ ] Database connection pooling
- [ ] OpenAPI/Swagger generation
- [ ] Request/Response compression
- [ ] Metrics/observability


## 🤝 Contributing

Contributions welcome! Please:

1. Maintain the zero-overhead philosophy
2. Keep binary size under 3 MB
3. Add benchmarks for new features
4. Update documentation
5. Ensure async safety

## 📄 License

MIT License - See LICENSE file

## 🙏 Acknowledgments

Built with:

- [Tokio](https://tokio.rs/) - Async runtime
- [Hyper](https://hyper.rs/) - HTTP library
- [Serde](https://serde.rs/) - Serialization framework
- [jemalloc](https://github.com/jemalloc/jemalloc) - Memory allocator
- [AHash](https://github.com/tkaitchuck/aHash) - Fast hashing
- [DashMap](https://github.com/xacrimon/dashmap) - Concurrent HashMap
- [notify](https://github.com/notify-rs/notify) - File watching
- [colored](https://github.com/mackwic/colored) - Terminal colors
- [chrono](https://github.com/chronotope/chrono) - Date/time handling

---

**Made with ⚡ and 🦀 for blazingly fast web development**
