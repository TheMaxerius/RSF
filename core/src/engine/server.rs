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
        #[cfg(feature = "use_hyper")]
        {
            use crate::engine::server_hyper::run_hyper;
            let socket_addr = addr.parse().expect("Invalid address");
            if let Err(e) = run_hyper(std::sync::Arc::new(handler), socket_addr).await {
                eprintln!("Server error: {}", e);
            }
        }

        #[cfg(not(feature = "use_hyper"))]
        {
            let listener = TcpListener::bind(&addr).await.unwrap();
            loop {
                let (mut socket, _) = listener.accept().await.unwrap();
                let handler = handler.clone();
                tokio::spawn(async move {
                    use tokio::time::{timeout, Duration};
                    // Read headers up to a reasonable limit (16kiB)
                    let mut buf = Vec::new();
                    let mut tmp = [0u8; 1024];
                    let header_limit = 16 * 1024;
                    // timeout to avoid slowloris
                    let read_result = timeout(Duration::from_secs(5), async {
                        loop {
                            let n = socket.read(&mut tmp).await.map_err(|e| e.to_string())?;
                            if n == 0 { break; }
                            buf.extend_from_slice(&tmp[..n]);
                            if buf.len() > header_limit { return Err("header too large".to_string()); }
                            if buf.windows(4).any(|w| w == b"\r\n\r\n") { break; }
                        }
                        Ok::<(), String>(())
                    }).await;

                    if read_result.is_err() {
                        let _ = socket.write_all(b"HTTP/1.1 408 Request Timeout\r\nConnection: close\r\n\r\n").await;
                        return;
                    }

                    if let Err(e) = read_result.unwrap() {
                        let _ = socket.write_all(format!("HTTP/1.1 400 Bad Request\r\nConnection: close\r\n\r\n{}", e).as_bytes()).await;
                        return;
                    }

                    let req = String::from_utf8_lossy(&buf);
                    let mut lines = req.lines();
                    let request_line = match lines.next() {
                        Some(l) => l,
                        None => return,
                    };
                    let parts: Vec<&str> = request_line.split_whitespace().collect();
                    if parts.len() < 2 {
                        let _ = socket.write_all(b"HTTP/1.1 400 Bad Request\r\nConnection: close\r\n\r\n").await;
                        return;
                    }
                    let method = parts[0];
                    let path = parts[1];

                        let response = handler.handle_request(method, path).await;
                        // build response head
                        let mut head = format!("HTTP/1.1 {} {}\r\nContent-Length: {}\r\nContent-Type: {}\r\n",
                            response.status,
                            crate::engine::handler::status_text(response.status),
                            response.body.len(),
                            response.content_type);
                        for (k, v) in &response.headers {
                            head.push_str(&format!("{}: {}\r\n", k, v));
                        }
                        head.push_str("Connection: close\r\n\r\n");
                        let _ = socket.write_all(head.as_bytes()).await;
                        let _ = socket.write_all(&response.body).await;
                });
            }
        }
    }
}
    