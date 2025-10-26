use std::convert::Infallible;
use std::sync::Arc;
use hyper::{Server, service::{make_service_fn, service_fn}, Body, Request as HyperRequest, Response as HyperResponse, Method, body::to_bytes};
use bytes::Bytes;
use crate::engine::handler::{RequestHandler, Response as MyResponse};

// Adapter: converts hyper requests to our RequestHandler and builds hyper responses.
pub async fn run_hyper(handler: Arc<RequestHandler>, addr: std::net::SocketAddr) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let make_svc = make_service_fn(move |_| {
        let handler = handler.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req: HyperRequest<Body>| {
                let handler = handler.clone();
                async move {
                    let method = req.method().as_str().to_string();
                    let path = req.uri().path().to_string();
                    
                    // Extract request body bytes
                    let (parts, body) = req.into_parts();
                    let body_bytes = match to_bytes(body).await {
                        Ok(bytes) => bytes,
                        Err(e) => {
                            log::error!("Failed to read request body: {}", e);
                            Bytes::new()
                        }
                    };
                    
                    // Pass body to handler
                    let my_resp = handler.handle_request_with_logging(&method, &path, body_bytes).await;
                    
                    // Build a proper Hyper response using the structured Response returned by handler
                    let content_length = my_resp.body.len();
                    let body = Body::from(my_resp.body);
                    let resp = HyperResponse::builder()
                        .status(my_resp.status)
                        .header("content-type", my_resp.content_type)
                        .header("content-length", content_length.to_string())
                        .body(body)
                        .unwrap_or_else(|_| HyperResponse::new(Body::from("Internal Server Error")));
                    Ok::<_, Infallible>(resp)
                }
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);
    server.await?;
    Ok(())
}
