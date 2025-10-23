use std::convert::Infallible;
use std::sync::Arc;
use hyper::{Server, service::{make_service_fn, service_fn}, Body, Request as HyperRequest, Response as HyperResponse, Method};
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
                    // For simplicity, ignore body for now
                    let resp = handler.handle_request(&method, &path).await;
                    // handler.handle_request currently returns raw HTTP string; if you migrate to
                    // returning structured types you can build HyperResponse directly. We'll wrap
                    // the raw response into an HTTP 200 with text/plain for compatibility.
                    Ok::<_, Infallible>(HyperResponse::builder().status(200).body(Body::from(resp)).unwrap())
                }
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);
    server.await?;
    Ok(())
}
