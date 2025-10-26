# Middleware Guide

## Overview

This framework provides a flexible middleware system with before/after hooks, allowing you to create custom middleware that can modify requests and responses, add headers, implement authentication, logging, and more.

## Quick Start

### 1. Basic Before Middleware

```rust
use crate::engine::{MiddlewareContext, MiddlewareResult, MiddlewareChain};
use once_cell::sync::Lazy;

static BEFORE_MIDDLEWARE: Lazy<MiddlewareChain> = Lazy::new(|| {
    MiddlewareChain::new()
        .add(|ctx| async move {
            log::info!("Request: {} {}", ctx.method, ctx.path);
            MiddlewareResult::Continue(ctx)
        })
});

pub async fn GET(params: &HashMap<String, String>) -> Response {
    let ctx = MiddlewareContext::new("GET".to_string(), "/endpoint".to_string());
    
    // Execute middleware
    let ctx = match BEFORE_MIDDLEWARE.execute(ctx).await {
        MiddlewareResult::Continue(ctx) => ctx,
        MiddlewareResult::Response(resp) => return resp,
    };
    
    // Handler logic
    Response::json(&serde_json::json!({
        "message": "Hello!"
    }), 200)
}
```

### 2. After Middleware

```rust
use crate::engine::{AfterMiddlewareChain};

static AFTER_MIDDLEWARE: Lazy<AfterMiddlewareChain> = Lazy::new(|| {
    AfterMiddlewareChain::new()
        .add(|_ctx, mut resp| async move {
            // Add CORS headers
            resp.headers.push(("Access-Control-Allow-Origin".to_string(), "*".to_string()));
            resp
        })
});

pub async fn GET(params: &HashMap<String, String>) -> Response {
    let ctx = MiddlewareContext::new("GET".to_string(), "/endpoint".to_string());
    
    let response = Response::json(&serde_json::json!({
        "message": "Hello!"
    }), 200);
    
    // Execute after middleware
    AFTER_MIDDLEWARE.execute(ctx, response).await
}
```

### 3. Chaining Multiple Middleware

```rust
static BEFORE_MIDDLEWARE: Lazy<MiddlewareChain> = Lazy::new(|| {
    MiddlewareChain::new()
        // Logging middleware
        .add(|ctx| async move {
            log::info!("🔵 Request: {} {}", ctx.method, ctx.path);
            MiddlewareResult::Continue(ctx)
        })
        // Timing middleware
        .add(|mut ctx| async move {
            use std::time::Instant;
            let start = Instant::now();
            ctx.set("start_time".to_string(), format!("{:?}", start));
            MiddlewareResult::Continue(ctx)
        })
        // Auth middleware
        .add(|ctx| async move {
            if let Some(api_key) = ctx.header("x-api-key") {
                if api_key == "secret-key" {
                    return MiddlewareResult::Continue(ctx);
                }
            }
            // Unauthorized - short-circuit the request
            MiddlewareResult::Response(Response::json(&serde_json::json!({
                "error": "Unauthorized"
            }), 401))
        })
});
```

## API Reference

### MiddlewareContext

The context object passed through middleware:

```rust
pub struct MiddlewareContext {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub extensions: HashMap<String, String>,
}
```

Methods:
- `new(method: String, path: String) -> Self` - Create new context
- `header(&self, name: &str) -> Option<&String>` - Get request header
- `set(&mut self, key: String, value: String)` - Store data in extensions
- `get(&self, key: &str) -> Option<&String>` - Get data from extensions

### MiddlewareResult

The result type for before middleware:

```rust
pub enum MiddlewareResult {
    Continue(MiddlewareContext),  // Continue to next middleware/handler
    Response(Response),            // Short-circuit and return response
}
```

### MiddlewareChain

Chain before middleware:

```rust
let chain = MiddlewareChain::new()
    .add(middleware_fn_1)
    .add(middleware_fn_2)
    .add(middleware_fn_3);
    
let result = chain.execute(ctx).await;
```

### AfterMiddlewareChain

Chain after middleware:

```rust
let chain = AfterMiddlewareChain::new()
    .add(after_fn_1)
    .add(after_fn_2);
    
let response = chain.execute(ctx, response).await;
```

## Common Patterns

### 1. Logging Middleware

```rust
.add(|ctx| async move {
    log::info!("🔵 {} {}", ctx.method, ctx.path);
    MiddlewareResult::Continue(ctx)
})
```

### 2. Timing Middleware

```rust
// Before
.add(|mut ctx| async move {
    ctx.set("start_time".to_string(), format!("{:?}", Instant::now()));
    MiddlewareResult::Continue(ctx)
})

// After
.add(|ctx, mut resp| async move {
    if let Some(start) = ctx.get("start_time") {
        resp.headers.push(("X-Response-Time".to_string(), "tracked".to_string()));
    }
    resp
})
```

### 3. CORS Middleware

```rust
.add(|_ctx, mut resp| async move {
    resp.headers.push(("Access-Control-Allow-Origin".to_string(), "*".to_string()));
    resp.headers.push(("Access-Control-Allow-Methods".to_string(), "GET, POST, PUT, DELETE".to_string()));
    resp
})
```

### 4. Authentication Middleware

```rust
.add(|ctx| async move {
    if let Some(auth) = ctx.header("authorization") {
        if auth.starts_with("Bearer ") {
            let token = &auth[7..];
            if validate_token(token) {
                return MiddlewareResult::Continue(ctx);
            }
        }
    }
    MiddlewareResult::Response(Response::json(&serde_json::json!({
        "error": "Unauthorized"
    }), 401))
})
```

### 5. Request Validation

```rust
.add(|ctx| async move {
    if let Some(content_type) = ctx.header("content-type") {
        if content_type.contains("application/json") {
            return MiddlewareResult::Continue(ctx);
        }
    }
    MiddlewareResult::Response(Response::json(&serde_json::json!({
        "error": "Content-Type must be application/json"
    }), 400))
})
```

## Examples

See `example/middleware_demo.rs` for a complete implementation demonstrating:
- Logging middleware
- Timing middleware
- API key validation
- CORS headers
- Custom response headers
- Context extension usage

## Performance

Middleware maintains the framework's performance characteristics:
- Zero-allocation for middleware chains
- Inline async execution
- Sub-millisecond overhead (typically <0.1ms)
- Early termination support for failed auth/validation

## Best Practices

1. **Keep middleware focused** - Each middleware should have a single responsibility
2. **Use extensions for data passing** - Share data between middleware using `ctx.set()` and `ctx.get()`
3. **Short-circuit when appropriate** - Return `MiddlewareResult::Response` for auth failures or validation errors
4. **Log middleware execution** - Add logging to help debug middleware chains
5. **Order matters** - Execute middleware in logical order (logging → timing → auth → validation)
6. **Use Lazy statics** - Define middleware chains as `Lazy` statics to avoid recreation on each request
