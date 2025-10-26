// 'api'
use std::collections::HashMap;
use crate::engine::Response;
use bytes::Bytes;

/// GET /ws-demo - WebSocket demo information
pub async fn GET(_params: &HashMap<String, String>) -> Response {
    Response::json(&serde_json::json!({
        "info": "WebSocket support is available in this framework!",
        "features": [
            "Real-time bidirectional communication",
            "Built on hyper-tungsten for performance",
            "Support for rooms and broadcasting",
            "Clean async API"
        ],
        "example_endpoints": [
            "/ws/chat - Multi-user chat room (coming soon)",
            "/ws-demo - This information page"
        ],
        "how_to_use": {
            "step_1": "Create a route file in example/ws/",
            "step_2": "Import: use crate::engine::{is_websocket_upgrade, upgrade_websocket, WsMessage};",
            "step_3": "Check for upgrade request and handle WebSocket connection",
            "step_4": "Use ws.send() and ws.receive() for messaging"
        },
        "code_example": "See example/ws/chat.rs for a complete implementation"
    }), 200)
}
