// 'api'
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use crate::engine::{Response, WsMessage, WsRoom};
use bytes::Bytes;
use once_cell::sync::Lazy;

// Global chat room shared across all connections
static CHAT_ROOM: Lazy<WsRoom> = Lazy::new(|| WsRoom::new());
static USER_COUNTER: AtomicU64 = AtomicU64::new(0);

/// GET /ws/chat - WebSocket chat information
/// To connect via WebSocket, use a WebSocket client library
/// Example: ws://localhost:5000/ws/chat
pub async fn GET(_params: &HashMap<String, String>) -> Response {
    Response::json(&serde_json::json!({
        "info": "WebSocket Chat Room",
        "active_users": CHAT_ROOM.count(),
        "connection_url": "ws://localhost:5000/ws/chat",
        "usage": "Use a WebSocket client to connect. Send text messages to chat with other users.",
        "example": "websocat ws://localhost:5000/ws/chat"
    }), 200)
}

async fn handle_chat_connection(
    ws_connection: crate::engine::WebSocketConnection,
    user_id: String,
) -> Result<(), String> {
    // This is a placeholder - full WebSocket chat implementation requires
    // server modifications to support WebSocket upgrades in routing
    ws_connection.handle(|mut ws| async move {
        // Send welcome message
        ws.send(format!("Welcome to the chat! Your ID: {}", user_id)).await?;
        ws.send(format!("Active users: {}", CHAT_ROOM.count() + 1)).await?;
    
    // Create a channel for broadcasting
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    CHAT_ROOM.join(user_id.clone(), tx);
    
    // Broadcast join message
    CHAT_ROOM.broadcast(format!("🟢 {} joined the chat", user_id));
    
    // Split socket into sender and receiver
    let (mut sender, mut receiver) = ws.split();
    
    // Spawn task to handle outgoing messages (broadcasts from other users)
    let send_user_id = user_id.clone();
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(msg).await.is_err() {
                break;
            }
        }
    });
    
    // Handle incoming messages from this user
    while let Ok(Some(msg)) = receiver.receive().await {
        match msg {
            WsMessage::Text(text) => {
                log::info!("[Chat] {}: {}", user_id, text);
                
                // Broadcast to all other users
                CHAT_ROOM.broadcast(format!("{}: {}", user_id, text));
            }
            WsMessage::Close => {
                log::info!("[Chat] {} disconnected", user_id);
                break;
            }
            _ => {}
        }
    }
    
        // Clean up
        CHAT_ROOM.leave(&user_id);
        CHAT_ROOM.broadcast(format!("🔴 {} left the chat", user_id));
        send_task.abort();
        
        Ok(())
    }).await
}
