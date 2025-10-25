/// WebSocket support
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{
    tungstenite::{Message, protocol::CloseFrame},
    WebSocketStream,
};
use tokio::net::TcpStream;
use std::sync::Arc;
use dashmap::DashMap;

/// WebSocket connection wrapper
pub struct WebSocket {
    inner: WebSocketStream<TcpStream>,
}

impl WebSocket {
    pub fn new(stream: WebSocketStream<TcpStream>) -> Self {
        Self { inner: stream }
    }
    
    /// Send a text message
    pub async fn send_text(&mut self, text: String) -> Result<(), String> {
        self.inner.send(Message::Text(text)).await
            .map_err(|e| format!("Failed to send message: {}", e))
    }
    
    /// Send binary data
    pub async fn send_binary(&mut self, data: Vec<u8>) -> Result<(), String> {
        self.inner.send(Message::Binary(data)).await
            .map_err(|e| format!("Failed to send message: {}", e))
    }
    
    /// Send ping
    pub async fn send_ping(&mut self) -> Result<(), String> {
        self.inner.send(Message::Ping(vec![])).await
            .map_err(|e| format!("Failed to send ping: {}", e))
    }
    
    /// Receive next message
    pub async fn receive(&mut self) -> Result<Option<WsMessage>, String> {
        match self.inner.next().await {
            Some(Ok(msg)) => Ok(Some(msg.into())),
            Some(Err(e)) => Err(format!("Receive error: {}", e)),
            None => Ok(None),
        }
    }
    
    /// Close connection
    pub async fn close(&mut self) -> Result<(), String> {
        self.inner.close(None).await
            .map_err(|e| format!("Failed to close: {}", e))
    }
    
    /// Close with frame
    pub async fn close_with(&mut self, code: u16, reason: String) -> Result<(), String> {
        let frame = CloseFrame {
            code: code.into(),
            reason: reason.into(),
        };
        self.inner.close(Some(frame)).await
            .map_err(|e| format!("Failed to close: {}", e))
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
            Message::Frame(_) => WsMessage::Close, // Treat raw frames as close
        }
    }
}

/// WebSocket room for broadcasting
pub struct WsRoom {
    connections: Arc<DashMap<String, tokio::sync::mpsc::UnboundedSender<String>>>,
}

impl WsRoom {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(DashMap::new()),
        }
    }
    
    /// Add connection to room
    pub fn join(&self, id: String, sender: tokio::sync::mpsc::UnboundedSender<String>) {
        self.connections.insert(id, sender);
    }
    
    /// Remove connection from room
    pub fn leave(&self, id: &str) {
        self.connections.remove(id);
    }
    
    /// Broadcast message to all connections
    pub fn broadcast(&self, message: String) {
        self.connections.retain(|_, sender| {
            sender.send(message.clone()).is_ok()
        });
    }
    
    /// Send to specific connection
    pub fn send_to(&self, id: &str, message: String) -> Result<(), String> {
        self.connections.get(id)
            .ok_or_else(|| "Connection not found".to_string())?
            .send(message)
            .map_err(|e| format!("Failed to send: {}", e))
    }
    
    /// Get number of connections
    pub fn count(&self) -> usize {
        self.connections.len()
    }
}

impl Clone for WsRoom {
    fn clone(&self) -> Self {
        Self {
            connections: Arc::clone(&self.connections),
        }
    }
}

impl Default for WsRoom {
    fn default() -> Self {
        Self::new()
    }
}

/// WebSocket handler trait
pub trait WsHandler: Send + Sync + 'static {
    fn on_connect(&self, id: String);
    fn on_message(&self, id: String, message: WsMessage);
    fn on_disconnect(&self, id: String);
}
