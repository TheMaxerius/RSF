// Server to handle requests (Api Calls and ui Requests)
// import the current Runtime

use crate::engine::runtime::Runtime;
use crate::engine::handler::RequestHandler;
use crate::logger;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// If the `http` crate isn't a dependency, provide a simple local HttpServer placeholder.
// Replace or remove this when integrating a real HTTP server implementation.
pub struct HttpServer {
    pub port: u16,
    pub host: String,
}

impl HttpServer {
    pub fn new(port: u16, host: &str) -> Self {
        HttpServer { port, host: host.to_string() }
    }
}

pub struct Server {
    pub runtime: Runtime,
    pub http_server: HttpServer,
    pub handler: RequestHandler,
}

impl Server {
    pub fn new(port: u16, host: String, dev: bool) -> Self {
        let runtime = Runtime::new(port, host.clone(), dev);
        let http_server = HttpServer::new(port, &host);
        let handler = RequestHandler::new(&runtime);
        Server { runtime, http_server, handler }
    }

    pub async fn start(self) {
    // Placeholder start function
    logger::info(&format!("Starting server at {}:{}", self.http_server.host, self.http_server.port));
        // Start HTTP server
        let addr = format!("{}:{}", self.http_server.host, self.http_server.port);
        let handler = self.handler.clone();

        let listener = TcpListener::bind(&addr).await.unwrap();
        loop {
            let (mut socket, _) = listener.accept().await.unwrap();
            let handler = handler.clone();
            tokio::spawn(async move {
                let mut buffer = [0; 1024];
                let n = socket.read(&mut buffer).await.unwrap();
                let request_str = String::from_utf8_lossy(&buffer[..n]);
                // Very naive HTTP request parsing
                let lines: Vec<&str> = request_str.lines().collect();
                if lines.is_empty() {
                    return;
                }
                let request_line = lines[0];
                let parts: Vec<&str> = request_line.split_whitespace().collect();
                if parts.len() < 2 {
                    return;
                }
                let method = parts[0];
                let path = parts[1];
                let response = handler.handle_request(method, path).await;
                let _ = socket.write_all(response.as_bytes()).await;
            });
        }
    }
}
    