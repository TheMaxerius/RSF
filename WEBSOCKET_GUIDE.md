# WebSocket Guide

## Overview

This framework includes built-in WebSocket support powered by `hyper-tungstenite`, providing real-time bidirectional communication with a clean, async API.

## Quick Start

### 1. Basic WebSocket Handler

Create a route file that handles WebSocket upgrades:

```rust
// 'api'
use std::collections::HashMap;
use crate::engine::{Response, is_websocket_upgrade, upgrade_websocket, WsMessage};
use hyper::{Request, Body};

pub async fn GET(params: &HashMap<String, String>, req: &mut Request<Body>) -> Response {
    // Check if this is a WebSocket upgrade request
    if !is_websocket_upgrade(req) {
        return Response::json(&serde_json::json!({
            "error": "This endpoint requires WebSocket upgrade"
        }), 400);
    }
    
    // Upgrade the connection
    let (response, ws_connection) = match upgrade_websocket(req) {
        Ok((resp, ws)) => (resp, ws),
        Err(e) => {
            return Response::json(&serde_json::json!({
                "error": format!("Upgrade failed: {}", e)
            }), 500);
        }
    };
    
    // Spawn a task to handle the WebSocket
    tokio::spawn(async move {
        if let Err(e) = handle_connection(ws_connection).await {
            log::error!("WebSocket error: {}", e);
        }
    });
    
    // Return the upgrade response
    Response {
        body: hyper::body::to_bytes(response.into_body()).await.unwrap_or_default(),
        status: response.status().as_u16(),
        content_type: "text/plain",
        headers: vec![],
    }
}

async fn handle_connection(ws_connection: crate::engine::WebSocketConnection) -> Result<(), String> {
    ws_connection.handle(|mut ws| async move {
        // Send a message
        ws.send("Hello from server!").await?;
        
        // Receive messages
        while let Ok(Some(msg)) = ws.receive().await {
            match msg {
                WsMessage::Text(text) => {
                    log::info!("Received: {}", text);
                    // Echo back
                    ws.send(format!("Echo: {}", text)).await?;
                }
                WsMessage::Close => break,
                _ => {}
            }
        }
        
        Ok(())
    }).await
}
```

### 2. Split API for Concurrent Send/Receive

For applications that need to send and receive simultaneously:

```rust
async fn handle_connection(ws_connection: crate::engine::WebSocketConnection) -> Result<(), String> {
    ws_connection.handle(|ws| async move {
        let (mut sender, mut receiver) = ws.split();
        
        // Spawn a task to handle outgoing messages
        let send_task = tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(5)).await;
                if sender.send("Ping").await.is_err() {
                    break;
                }
            }
        });
        
        // Handle incoming messages
        while let Ok(Some(msg)) = receiver.receive().await {
            match msg {
                WsMessage::Text(text) => {
                    log::info!("Received: {}", text);
                }
                WsMessage::Close => break,
                _ => {}
            }
        }
        
        send_task.abort();
        Ok(())
    }).await
}
```

### 3. Broadcasting with WsRoom

For multi-user applications like chat rooms:

```rust
use once_cell::sync::Lazy;
use crate::engine::WsRoom;

static CHAT_ROOM: Lazy<WsRoom> = Lazy::new(|| WsRoom::new());

async fn handle_chat(ws_connection: crate::engine::WebSocketConnection, user_id: String) -> Result<(), String> {
    ws_connection.handle(|ws| async move {
        // Create a channel for broadcasts
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        CHAT_ROOM.join(user_id.clone(), tx);
        
        // Broadcast join message
        CHAT_ROOM.broadcast(format!("{} joined", user_id));
        
        let (mut sender, mut receiver) = ws.split();
        
        // Handle broadcasts from other users
        let send_task = tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if sender.send(msg).await.is_err() {
                    break;
                }
            }
        });
        
        // Handle incoming messages
        while let Ok(Some(msg)) = receiver.receive().await {
            match msg {
                WsMessage::Text(text) => {
                    CHAT_ROOM.broadcast(format!("{}: {}", user_id, text));
                }
                WsMessage::Close => break,
                _ => {}
            }
        }
        
        CHAT_ROOM.leave(&user_id);
        send_task.abort();
        Ok(())
    }).await
}
```

## API Reference

### Core Functions

- `is_websocket_upgrade(req: &Request<Body>) -> bool` - Check if request is a WebSocket upgrade
- `upgrade_websocket(req: &mut Request<Body>) -> Result<(Response<Body>, WebSocketConnection), String>` - Upgrade HTTP connection to WebSocket

### WebSocketConnection

- `handle<F, Fut>(self, handler: F) -> Result<(), String>` - Handle the WebSocket with a custom async handler
- `into_websocket(self) -> Result<WebSocket, String>` - Convert to WebSocket directly

### WebSocket

- `send(&mut self, text: impl Into<String>) -> Result<(), String>` - Send text message
- `send_binary(&mut self, data: Vec<u8>) -> Result<(), String>` - Send binary message
- `receive(&mut self) -> Result<Option<WsMessage>, String>` - Receive next message
- `split(self) -> (WsSender, WsReceiver)` - Split into sender and receiver for concurrent operations
- `close(self) -> Result<(), String>` - Close the WebSocket connection

### WsRoom

- `new() -> Self` - Create a new room
- `join(&self, id: String, sender: UnboundedSender<String>)` - Add connection to room
- `leave(&self, id: &str)` - Remove connection from room
- `broadcast(&self, message: impl Into<String>)` - Send message to all connections
- `send_to(&self, id: &str, message: impl Into<String>)` - Send message to specific connection
- `count() -> usize` - Get number of active connections

## Message Types

```rust
pub enum WsMessage {
    Text(String),      // Text message
    Binary(Vec<u8>),   // Binary data
    Ping(Vec<u8>),     // Ping frame
    Pong(Vec<u8>),     // Pong frame
    Close,             // Connection close
}
```

## Examples

See `example/ws/chat.rs` for a complete chat room implementation demonstrating:
- WebSocket upgrade handling
- Room-based broadcasting
- Split sender/receiver pattern
- Connection lifecycle management

## Performance

WebSocket support maintains the framework's performance characteristics:
- Zero-copy message passing where possible
- Async/await for concurrent connections
- Sub-millisecond response times for upgrades
