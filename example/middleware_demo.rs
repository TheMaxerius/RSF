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
