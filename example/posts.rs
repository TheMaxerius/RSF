// 'api'
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use crate::engine::{Response, Json};
use bytes::Bytes;
use once_cell::sync::Lazy;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Post {
    pub id: usize,
    pub title: String,
    pub content: String,
    pub author: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
struct CreatePostRequest {
    title: String,
    content: String,
    author: String,
}

// Global in-memory database
pub static POSTS: Lazy<Arc<Mutex<Vec<Post>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(vec![
        Post {
            id: 1,
            title: "Welcome to the Blog API".to_string(),
            content: "This is a blazingly fast blog API built with Rust! Features include async handlers, type-safe routing, and sub-millisecond response times.".to_string(),
            author: "Admin".to_string(),
            created_at: "2025-10-26T08:00:00Z".to_string(),
            updated_at: "2025-10-26T08:00:00Z".to_string(),
        },
        Post {
            id: 2,
            title: "Why Rust for Web Development?".to_string(),
            content: "Rust provides memory safety, zero-cost abstractions, and incredible performance. Perfect for building fast APIs!".to_string(),
            author: "Alice".to_string(),
            created_at: "2025-10-26T09:00:00Z".to_string(),
            updated_at: "2025-10-26T09:00:00Z".to_string(),
        },
        Post {
            id: 3,
            title: "Async/Await in Rust".to_string(),
            content: "Rust's async/await syntax makes it easy to write concurrent code. This API uses Tokio for async runtime!".to_string(),
            author: "Bob".to_string(),
            created_at: "2025-10-26T10:00:00Z".to_string(),
            updated_at: "2025-10-26T10:00:00Z".to_string(),
        },
    ]))
});

/// GET /posts - List all posts
pub async fn GET(_params: &HashMap<String, String>) -> Response {
    let posts = POSTS.lock().unwrap();
    Response::json(&*posts, 200)
}

/// POST /posts - Create a new post
pub async fn POST(_params: &HashMap<String, String>, body: &Bytes) -> Response {
    // Parse JSON body
    let request: Json<CreatePostRequest> = match Json::from_bytes(body) {
        Ok(json) => json,
        Err(e) => {
            return Response::json(&serde_json::json!({
                "error": "Invalid JSON body",
                "details": e
            }), 400);
        }
    };

    let mut posts = POSTS.lock().unwrap();
    let next_id = posts.iter().map(|p| p.id).max().unwrap_or(0) + 1;
    
    let now = chrono::Utc::now().to_rfc3339();
    let new_post = Post {
        id: next_id,
        title: request.0.title,
        content: request.0.content,
        author: request.0.author,
        created_at: now.clone(),
        updated_at: now,
    };
    
    posts.push(new_post.clone());
    Response::json(&new_post, 201)
}
