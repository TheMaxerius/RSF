/// Modern WebSocket support with hyper-tungstenite
use hyper::{Request, Response, Body, StatusCode, header};
use hyper_tungstenite::{HyperWebsocket, is_upgrade_request as is_ws_upgrade};
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::Message;
use std::sync::Arc;
use dashmap::DashMap;

/// WebSocket connection wrapper for user handlers
pub struct WebSocketConnection {
    socket: HyperWebsocket,
}

impl WebSocketConnection {
    pub fn new(socket: HyperWebsocket) -> Self {
        Self { socket }
    }
    
    /// Await the WebSocket upgrade and get the WebSocket
    pub async fn into_websocket(self) -> Result<WebSocket, String> {
        let ws_stream = self.socket.await
            .map_err(|e| format!("WebSocket handshake failed: {}", e))?;
        
        Ok(WebSocket { inner: ws_stream })
    }
    
    /// Handle WebSocket connection with user-defined async handler
    pub async fn handle<F, Fut>(self, handler: F) -> Result<(), String>
    where
        F: FnOnce(WebSocket) -> Fut,
        Fut: std::future::Future<Output = Result<(), String>>,
    {
        let ws = self.into_websocket().await?;
        handler(ws).await
    }
}

/// WebSocket for reading and writing messages
pub struct WebSocket {
    inner: hyper_tungstenite::WebSocketStream<hyper::upgrade::Upgraded>,
}

impl WebSocket {
    /// Send a text message
    pub async fn send(&mut self, text: impl Into<String>) -> Result<(), String> {
        self.inner
            .send(Message::Text(text.into()))
            .await
            .map_err(|e| format!("Send error: {}", e))
    }
    
    /// Send binary data
    pub async fn send_binary(&mut self, data: Vec<u8>) -> Result<(), String> {
        self.inner
            .send(Message::Binary(data))
            .await
            .map_err(|e| format!("Send error: {}", e))
    }
    
    /// Receive the next message
    pub async fn receive(&mut self) -> Result<Option<WsMessage>, String> {
        match self.inner.next().await {
            Some(Ok(msg)) => Ok(Some(msg.into())),
            Some(Err(e)) => Err(format!("Receive error: {}", e)),
            None => Ok(None),
        }
    }
    
    /// Split into separate sender and receiver
    pub fn split(self) -> (WsSender, WsReceiver) {
        let (sink, stream) = self.inner.split();
        (WsSender { sink }, WsReceiver { stream })
    }
    
    /// Close the connection
    pub async fn close(mut self) -> Result<(), String> {
        self.inner
            .close(None)
            .await
            .map_err(|e| format!("Close error: {}", e))
    }
}

/// WebSocket sender (for split connections)
pub struct WsSender {
    sink: futures_util::stream::SplitSink<
        hyper_tungstenite::WebSocketStream<hyper::upgrade::Upgraded>,
        Message,
    >,
}

impl WsSender {
    pub async fn send(&mut self, text: impl Into<String>) -> Result<(), String> {
        self.sink
            .send(Message::Text(text.into()))
            .await
            .map_err(|e| format!("Send error: {}", e))
    }
    
    pub async fn send_binary(&mut self, data: Vec<u8>) -> Result<(), String> {
        self.sink
            .send(Message::Binary(data))
            .await
            .map_err(|e| format!("Send error: {}", e))
    }
}

/// WebSocket receiver (for split connections)
pub struct WsReceiver {
    stream: futures_util::stream::SplitStream<
        hyper_tungstenite::WebSocketStream<hyper::upgrade::Upgraded>,
    >,
}

impl WsReceiver {
    pub async fn receive(&mut self) -> Result<Option<WsMessage>, String> {
        match self.stream.next().await {
            Some(Ok(msg)) => Ok(Some(msg.into())),
            Some(Err(e)) => Err(format!("Receive error: {}", e)),
            None => Ok(None),
        }
    }
}

/// WebSocket message types
#[derive(Debug, Clone)]
pub enum WsMessage {
    Text(String),
    Binary(Vec<u8>),
    Ping(Vec<u8>),
    Pong(Vec<u8>),
    Close,
}

impl From<Message> for WsMessage {
    fn from(msg: Message) -> Self {
        match msg {
            Message::Text(t) => WsMessage::Text(t),
            Message::Binary(b) => WsMessage::Binary(b),
            Message::Ping(p) => WsMessage::Ping(p),
            Message::Pong(p) => WsMessage::Pong(p),
            Message::Close(_) => WsMessage::Close,
            Message::Frame(_) => WsMessage::Close,
        }
    }
}

/// Check if a request is a WebSocket upgrade request
pub fn is_websocket_upgrade(req: &Request<Body>) -> bool {
    is_ws_upgrade(req)
}

/// Upgrade HTTP request to WebSocket
pub fn upgrade_websocket(req: &mut Request<Body>) -> Result<(Response<Body>, WebSocketConnection), String> {
    match hyper_tungstenite::upgrade(req, None) {
        Ok((response, websocket)) => {
            Ok((response, WebSocketConnection::new(websocket)))
        }
        Err(e) => Err(format!("WebSocket upgrade failed: {}", e)),
    }
}

/// WebSocket room for broadcasting messages
#[derive(Clone)]
pub struct WsRoom {
    connections: Arc<DashMap<String, tokio::sync::mpsc::UnboundedSender<String>>>,
}

impl WsRoom {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(DashMap::new()),
        }
    }
    
    /// Add a connection to the room
    pub fn join(&self, id: String, sender: tokio::sync::mpsc::UnboundedSender<String>) {
        self.connections.insert(id, sender);
    }
    
    /// Remove a connection from the room
    pub fn leave(&self, id: &str) {
        self.connections.remove(id);
    }
    
    /// Broadcast message to all connections in the room
    pub fn broadcast(&self, message: impl Into<String>) {
        let msg = message.into();
        self.connections.retain(|_, sender| {
            sender.send(msg.clone()).is_ok()
        });
    }
    
    /// Send message to a specific connection
    pub fn send_to(&self, id: &str, message: impl Into<String>) -> Result<(), String> {
        self.connections
            .get(id)
            .ok_or_else(|| "Connection not found".to_string())?
            .send(message.into())
            .map_err(|e| format!("Failed to send: {}", e))
    }
    
    /// Get the number of active connections
    pub fn count(&self) -> usize {
        self.connections.len()
    }
    
    /// Get all connection IDs
    pub fn connection_ids(&self) -> Vec<String> {
        self.connections.iter().map(|entry| entry.key().clone()).collect()
    }
}

impl Default for WsRoom {
    fn default() -> Self {
        Self::new()
    }
}
