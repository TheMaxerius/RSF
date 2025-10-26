// 'api'
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::engine::{Response, Json};
use bytes::Bytes;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Post {
    id: usize,
    title: String,
    content: String,
    author: String,
}

#[derive(Deserialize)]
struct UpdatePostRequest {
    title: String,
    content: String,
    author: String,
}

/// GET /posts/:id - Get a specific post
pub async fn GET(params: &HashMap<String, String>) -> Response {
    let id: usize = match params.get("id") {
        Some(id_str) => match id_str.parse() {
            Ok(n) => n,
            Err(_) => {
                return Response::json(&serde_json::json!({
                    "error": "Invalid post ID"
                }), 400);
            }
        },
        None => {
            return Response::json(&serde_json::json!({
                "error": "Missing post ID"
            }), 400);
        }
    };

    // For demo purposes, return a mock post
    let post = Post {
        id,
        title: format!("Post #{}", id),
        content: format!("This is the content of post {}. In a real app, this would come from a database.", id),
        author: "Demo Author".to_string(),
    };
    
    Response::json(&post, 200)
}

/// PUT /posts/:id - Update a post
pub async fn PUT(params: &HashMap<String, String>, body: &Bytes) -> Response {
    let id: usize = match params.get("id") {
        Some(id_str) => match id_str.parse() {
            Ok(n) => n,
            Err(_) => {
                return Response::json(&serde_json::json!({
                    "error": "Invalid post ID"
                }), 400);
            }
        },
        None => {
            return Response::json(&serde_json::json!({
                "error": "Missing post ID"
            }), 400);
        }
    };

    // Parse JSON body
    let request: Json<UpdatePostRequest> = match Json::from_bytes(body) {
        Ok(json) => json,
        Err(e) => {
            return Response::json(&serde_json::json!({
                "error": "Invalid JSON body",
                "details": e
            }), 400);
        }
    };

    let updated_post = Post {
        id,
        title: request.0.title,
        content: request.0.content,
        author: request.0.author,
    };
    
    Response::json(&serde_json::json!({
        "message": "Post updated successfully",
        "post": updated_post
    }), 200)
}

/// DELETE /posts/:id - Delete a post
pub async fn DELETE(params: &HashMap<String, String>) -> Response {
    let id: usize = match params.get("id") {
        Some(id_str) => match id_str.parse() {
            Ok(n) => n,
            Err(_) => {
                return Response::json(&serde_json::json!({
                    "error": "Invalid post ID"
                }), 400);
            }
        },
        None => {
            return Response::json(&serde_json::json!({
                "error": "Missing post ID"
            }), 400);
        }
    };

    Response::json(&serde_json::json!({
        "message": format!("Post {} deleted successfully", id),
        "deleted_id": id
    }), 200)
}
