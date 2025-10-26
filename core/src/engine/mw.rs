/// User-friendly middleware system with before/after hooks
use hyper::{Request, Response, Body, StatusCode};
use std::future::Future;
use std::pin::Pin;
use bytes::Bytes;
use std::collections::HashMap;

/// Middleware context that flows through the pipeline
#[derive(Clone)]
pub struct MiddlewareContext {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub extensions: HashMap<String, String>,
}

impl MiddlewareContext {
    pub fn new(method: String, path: String) -> Self {
        Self {
            method,
            path,
            headers: HashMap::new(),
            extensions: HashMap::new(),
        }
    }
    
    /// Get a value from extensions
    pub fn get(&self, key: &str) -> Option<&String> {
        self.extensions.get(key)
    }
    
    /// Set a value in extensions
    pub fn set(&mut self, key: String, value: String) {
        self.extensions.insert(key, value);
    }
    
    /// Get a header value
    pub fn header(&self, key: &str) -> Option<&String> {
        self.headers.get(key)
    }
}

/// Middleware response - can short-circuit the pipeline
pub enum MiddlewareResult {
    /// Continue to next middleware/handler
    Continue(MiddlewareContext),
    /// Short-circuit with a response
    Response(crate::engine::Response),
}

/// Middleware function signature
pub type MiddlewareFn = Box<
    dyn Fn(MiddlewareContext) -> Pin<Box<dyn Future<Output = MiddlewareResult> + Send>>
        + Send
        + Sync
>;

/// Middleware builder for chaining middleware
pub struct MiddlewareChain {
    middleware: Vec<MiddlewareFn>,
}

impl MiddlewareChain {
    pub fn new() -> Self {
        Self {
            middleware: Vec::new(),
        }
    }
    
    /// Add middleware to the chain
    pub fn add<F, Fut>(mut self, middleware: F) -> Self
    where
        F: Fn(MiddlewareContext) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = MiddlewareResult> + Send + 'static,
    {
        self.middleware.push(Box::new(move |ctx| {
            Box::pin(middleware(ctx))
        }));
        self
    }
    
    /// Execute the middleware chain
    pub async fn execute(&self, ctx: MiddlewareContext) -> MiddlewareResult {
        let mut current_ctx = ctx;
        
        for middleware in &self.middleware {
            match middleware(current_ctx.clone()).await {
                MiddlewareResult::Continue(ctx) => {
                    current_ctx = ctx;
                }
                MiddlewareResult::Response(resp) => {
                    return MiddlewareResult::Response(resp);
                }
            }
        }
        
        MiddlewareResult::Continue(current_ctx)
    }
}

impl Default for MiddlewareChain {
    fn default() -> Self {
        Self::new()
    }
}

/// After-response middleware for logging, metrics, etc.
pub type AfterMiddlewareFn = Box<
    dyn Fn(MiddlewareContext, crate::engine::Response) -> Pin<Box<dyn Future<Output = crate::engine::Response> + Send>>
        + Send
        + Sync
>;

/// After-response middleware chain
pub struct AfterMiddlewareChain {
    middleware: Vec<AfterMiddlewareFn>,
}

impl AfterMiddlewareChain {
    pub fn new() -> Self {
        Self {
            middleware: Vec::new(),
        }
    }
    
    /// Add after-response middleware
    pub fn add<F, Fut>(mut self, middleware: F) -> Self
    where
        F: Fn(MiddlewareContext, crate::engine::Response) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = crate::engine::Response> + Send + 'static,
    {
        self.middleware.push(Box::new(move |ctx, resp| {
            Box::pin(middleware(ctx, resp))
        }));
        self
    }
    
    /// Execute the after middleware chain
    pub async fn execute(&self, ctx: MiddlewareContext, mut response: crate::engine::Response) -> crate::engine::Response {
        for middleware in &self.middleware {
            response = middleware(ctx.clone(), response).await;
        }
        response
    }
}

impl Default for AfterMiddlewareChain {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Built-in middleware examples
// ============================================================================

/// Logging middleware
pub async fn logging_middleware(ctx: MiddlewareContext) -> MiddlewareResult {
    log::info!("[→] {} {}", ctx.method, ctx.path);
    MiddlewareResult::Continue(ctx)
}

/// CORS middleware
pub fn cors_middleware(allowed_origin: String) -> impl Fn(MiddlewareContext, crate::engine::Response) -> Pin<Box<dyn Future<Output = crate::engine::Response> + Send>> + Send + Sync + 'static {
    move |_ctx, mut resp| {
        let origin = allowed_origin.clone();
        Box::pin(async move {
            resp.headers.push(("Access-Control-Allow-Origin".to_string(), origin));
            resp.headers.push(("Access-Control-Allow-Methods".to_string(), "GET, POST, PUT, DELETE, OPTIONS".to_string()));
            resp.headers.push(("Access-Control-Allow-Headers".to_string(), "Content-Type, Authorization".to_string()));
            resp
        })
    }
}

/// Timing middleware
pub fn timing_middleware() -> (
    impl Fn(MiddlewareContext) -> Pin<Box<dyn Future<Output = MiddlewareResult> + Send>> + Send + Sync + 'static,
    impl Fn(MiddlewareContext, crate::engine::Response) -> Pin<Box<dyn Future<Output = crate::engine::Response> + Send>> + Send + Sync + 'static,
) {
    use std::time::Instant;
    
    let before = move |mut ctx: MiddlewareContext| {
        Box::pin(async move {
            let start = Instant::now();
            ctx.set("start_time".to_string(), format!("{:?}", start));
            MiddlewareResult::Continue(ctx)
        }) as Pin<Box<dyn Future<Output = MiddlewareResult> + Send>>
    };
    
    let after = move |ctx: MiddlewareContext, mut resp: crate::engine::Response| {
        Box::pin(async move {
            if let Some(start_str) = ctx.get("start_time") {
                // In production, parse start_time properly
                resp.headers.push(("X-Response-Time".to_string(), "tracked".to_string()));
            }
            resp
        }) as Pin<Box<dyn Future<Output = crate::engine::Response> + Send>>
    };
    
    (before, after)
}

/// Authentication middleware example
pub fn auth_middleware(required_token: String) -> impl Fn(MiddlewareContext) -> Pin<Box<dyn Future<Output = MiddlewareResult> + Send>> + Send + Sync + 'static {
    move |ctx| {
        let token = required_token.clone();
        Box::pin(async move {
            // Check for Authorization header
            if let Some(auth_header) = ctx.header("authorization") {
                if auth_header.contains(&token) {
                    return MiddlewareResult::Continue(ctx);
                }
            }
            
            // Return 401 if auth fails
            MiddlewareResult::Response(crate::engine::Response {
                body: bytes::Bytes::from(r#"{"error":"Unauthorized"}"#),
                status: 401,
                content_type: "application/json",
                headers: vec![],
            })
        }) as Pin<Box<dyn Future<Output = MiddlewareResult> + Send>>
    }
}
