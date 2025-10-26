/// Helper functions for improved developer experience
use super::{Response, MiddlewareContext, MiddlewareResult, MiddlewareChain, AfterMiddlewareChain, AppError};
use std::collections::HashMap;

/// Wrap a handler with before middleware
pub async fn with_middleware<F, Fut>(
    method: String,
    path: String,
    middleware: &MiddlewareChain,
    handler: F,
) -> Response
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Response>,
{
    let ctx = MiddlewareContext::new(method, path);
    
    match middleware.execute(ctx).await {
        MiddlewareResult::Continue(_) => handler().await,
        MiddlewareResult::Response(resp) => resp,
    }
}

/// Wrap a handler with before and after middleware
pub async fn with_middleware_chain<F, Fut>(
    method: String,
    path: String,
    before: &MiddlewareChain,
    after: &AfterMiddlewareChain,
    handler: F,
) -> Response
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Response>,
{
    let ctx = MiddlewareContext::new(method, path);
    
    let ctx = match before.execute(ctx).await {
        MiddlewareResult::Continue(ctx) => ctx,
        MiddlewareResult::Response(resp) => return resp,
    };
    
    let response = handler().await;
    after.execute(ctx, response).await
}

/// Convert Result<Response, AppError> to Response
pub fn result_to_response(result: Result<Response, AppError>) -> Response {
    match result {
        Ok(resp) => resp,
        Err(err) => err.into_response(),
    }
}
