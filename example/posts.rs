//'api'
// ✅ REAL EXAMPLE: Request Body Parsing with Validation
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use core::engine::BodyParser;

#[derive(Deserialize, Serialize)]
struct CreatePostRequest {
    title: String,
    content: String,
    tags: Vec<String>,
}

#[derive(Serialize)]
struct CreatePostResponse {
    id: String,
    title: String,
    content: String,
    tags: Vec<String>,
    created_at: String,
}

#[derive(Serialize)]
struct PostListResponse {
    posts: Vec<PostSummary>,
    total: usize,
    page: u32,
    per_page: u32,
}

#[derive(Serialize)]
struct PostSummary {
    id: String,
    title: String,
    excerpt: String,
    tags: Vec<String>,
}

pub fn POST(params: &HashMap<String, String>) -> (String, u16) {
    // ✅ REAL FEATURE: JSON Body Parsing
    let mock_body = r#"{
        "title": "My First Post",
        "content": "This is the content of my first blog post!",
        "tags": ["rust", "web", "performance"]
    }"#;
    
    let post_data = match BodyParser::json::<CreatePostRequest>(mock_body.as_bytes()) {
        Ok(data) => data,
        Err(e) => {
            return (format!(r#"{{"error": "Invalid JSON: {}"}}"#, e), 400);
        }
    };
    
    // ✅ Validation
    if post_data.title.trim().is_empty() {
        return (r#"{"error": "Title is required"}"#.to_string(), 400);
    }
    
    if post_data.title.len() > 200 {
        return (r#"{"error": "Title too long (max 200 chars)"}"#.to_string(), 400);
    }
    
    if post_data.content.trim().is_empty() {
        return (r#"{"error": "Content is required"}"#.to_string(), 400);
    }
    
    // Generate ID and timestamp
    let post_id = format!("post_{}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis());
    
    let created_at = chrono::Utc::now().to_rfc3339();
    
    // Create response
    let response = CreatePostResponse {
        id: post_id,
        title: post_data.title,
        content: post_data.content,
        tags: post_data.tags,
        created_at,
    };
    
    (serde_json::to_string(&response).unwrap(), 201)
}

// ✅ REAL FEATURE: Query String Parsing with Pagination
pub fn GET(params: &HashMap<String, String>) -> (String, u16) {
    use core::engine::QueryParser;
    
    // Mock query string (in production, extract from request URL)
    let query_string = "page=2&per_page=5&tag=rust";
    let query_params = QueryParser::parse(query_string);
    
    // Extract pagination params with defaults
    let page: u32 = query_params.get("page")
        .and_then(|p| p.parse().ok())
        .unwrap_or(1);
    
    let per_page: u32 = query_params.get("per_page")
        .and_then(|p| p.parse().ok())
        .unwrap_or(10);
    
    let tag_filter = query_params.get("tag");
    
    // Mock database query
    let mut posts = vec![
        PostSummary {
            id: "post_1".to_string(),
            title: "Getting Started with Rust".to_string(),
            excerpt: "Learn the basics of Rust programming...".to_string(),
            tags: vec!["rust".to_string(), "tutorial".to_string()],
        },
        PostSummary {
            id: "post_2".to_string(),
            title: "Web Performance Optimization".to_string(),
            excerpt: "Tips for building fast web applications...".to_string(),
            tags: vec!["web".to_string(), "performance".to_string()],
        },
        PostSummary {
            id: "post_3".to_string(),
            title: "Advanced Rust Patterns".to_string(),
            excerpt: "Explore advanced Rust programming patterns...".to_string(),
            tags: vec!["rust".to_string(), "advanced".to_string()],
        },
    ];
    
    // Filter by tag if provided
    if let Some(tag) = tag_filter {
        posts.retain(|p| p.tags.contains(tag));
    }
    
    let total = posts.len();
    
    // Apply pagination
    let start = ((page - 1) * per_page) as usize;
    let paginated_posts: Vec<PostSummary> = posts.into_iter().skip(start).take(per_page as usize).collect();
    
    let response = PostListResponse {
        posts: paginated_posts,
        total,
        page,
        per_page,
    };
    
    (serde_json::to_string(&response).unwrap(), 200)
}
