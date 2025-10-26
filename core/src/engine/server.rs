// Server to handle requests using Hyper
use crate::engine::runtime::Runtime;
use crate::engine::handler::RequestHandler;
use crate::engine::server_hyper::run_hyper;
use std::sync::Arc;

pub struct Server {
    pub runtime: Runtime,
    pub handler: RequestHandler,
    pub host: String,
    pub port: u16,
}

impl Server {
    pub fn new(port: u16, host: String, dev: bool) -> Self {
        let runtime = Runtime::new(port, host.clone(), dev);
        let handler = RequestHandler::new(&runtime);
        Server { 
            runtime, 
            handler,
            host,
            port,
        }
    }

    pub async fn start(self) {
        log::info!("Starting Hyper server at {}:{}", self.host, self.port);
        let addr = format!("{}:{}", self.host, self.port);
        let socket_addr = addr.parse().expect("Invalid address");
        
        if let Err(e) = run_hyper(Arc::new(self.handler), socket_addr).await {
            log::error!("Hyper server error: {}", e);
        }
    }
}
