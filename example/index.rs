// 'ui'
use std::collections::HashMap;
use crate::engine::Response;
use bytes::Bytes;

pub async fn GET(_params: &HashMap<String, String>) -> Response {
    let html = r#"<!DOCTYPE html>
<html>
<head>
    <title>Rust Blog API</title>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            padding: 2rem;
            color: #333;
        }
        .container {
            max-width: 900px;
            margin: 0 auto;
            background: white;
            border-radius: 12px;
            padding: 3rem;
            box-shadow: 0 20px 60px rgba(0,0,0,0.3);
        }
        h1 {
            color: #667eea;
            font-size: 2.5rem;
            margin-bottom: 1rem;
        }
        h2 {
            color: #764ba2;
            font-size: 1.5rem;
            margin: 2rem 0 1rem 0;
            border-bottom: 2px solid #eee;
            padding-bottom: 0.5rem;
        }
        .tagline {
            color: #666;
            font-size: 1.2rem;
            margin-bottom: 2rem;
        }
        .features {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 1rem;
            margin: 2rem 0;
        }
        .feature {
            background: #f8f9fa;
            padding: 1rem;
            border-radius: 8px;
            border-left: 4px solid #667eea;
        }
        .feature h3 {
            color: #667eea;
            font-size: 1rem;
            margin-bottom: 0.5rem;
        }
        .feature p {
            color: #666;
            font-size: 0.9rem;
        }
        .endpoint {
            background: #f8f9fa;
            padding: 1rem;
            margin: 0.5rem 0;
            border-radius: 6px;
            border-left: 4px solid #764ba2;
        }
        .method {
            display: inline-block;
            padding: 0.2rem 0.6rem;
            border-radius: 4px;
            font-weight: bold;
            font-size: 0.85rem;
            margin-right: 0.5rem;
        }
        .get { background: #28a745; color: white; }
        .post { background: #007bff; color: white; }
        .put { background: #ffc107; color: #333; }
        .delete { background: #dc3545; color: white; }
        code {
            background: #2d2d2d;
            color: #f8f8f2;
            padding: 0.2rem 0.4rem;
            border-radius: 3px;
            font-size: 0.9rem;
        }
        .example {
            background: #2d2d2d;
            color: #f8f8f2;
            padding: 1rem;
            border-radius: 6px;
            margin: 1rem 0;
            overflow-x: auto;
        }
        .example pre {
            margin: 0;
            font-size: 0.9rem;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>🚀 Rust Blog API</h1>
        <p class="tagline">A blazingly fast, type-safe web framework with async handlers</p>

        <div class="features">
            <div class="feature">
                <h3>⚡ Sub-millisecond</h3>
                <p>0.03-0.15ms response times</p>
            </div>
            <div class="feature">
                <h3>🔒 Type-Safe</h3>
                <p>Compile-time route generation</p>
            </div>
            <div class="feature">
                <h3>🔄 Async</h3>
                <p>Full async/await support</p>
            </div>
            <div class="feature">
                <h3>📦 Tiny</h3>
                <p>2.5 MB release binary</p>
            </div>
        </div>

        <h2>📚 API Endpoints</h2>

        <div class="endpoint">
            <span class="method get">GET</span>
            <code>/posts</code>
            <p style="margin-top: 0.5rem; color: #666;">List all blog posts</p>
        </div>

        <div class="endpoint">
            <span class="method get">GET</span>
            <code>/posts/:id</code>
            <p style="margin-top: 0.5rem; color: #666;">Get a specific post by ID</p>
        </div>

        <div class="endpoint">
            <span class="method post">POST</span>
            <code>/posts</code>
            <p style="margin-top: 0.5rem; color: #666;">Create a new post (requires JSON body)</p>
        </div>

        <div class="endpoint">
            <span class="method put">PUT</span>
            <code>/posts/:id</code>
            <p style="margin-top: 0.5rem; color: #666;">Update an existing post (requires JSON body)</p>
        </div>

        <div class="endpoint">
            <span class="method delete">DELETE</span>
            <code>/posts/:id</code>
            <p style="margin-top: 0.5rem; color: #666;">Delete a post</p>
        </div>

        <div class="endpoint">
            <span class="method get">GET</span>
            <code>/stats</code>
            <p style="margin-top: 0.5rem; color: #666;">View framework statistics</p>
        </div>

        <h2>💡 Example Usage</h2>

        <div class="example">
            <pre># Get all posts
curl http://localhost:5000/posts

# Get post by ID
curl http://localhost:5000/posts/1

# Create a new post
curl -X POST http://localhost:5000/posts \
  -H "Content-Type: application/json" \
  -d '{
    "title": "My First Post",
    "content": "Hello, World!",
    "author": "Alice"
  }'

# Update a post
curl -X PUT http://localhost:5000/posts/1 \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Updated Title",
    "content": "Updated content",
    "author": "Alice"
  }'

# Delete a post
curl -X DELETE http://localhost:5000/posts/1</pre>
        </div>

        <p style="margin-top: 2rem; color: #666; text-align: center;">
            Built with ❤️ using Rust, Tokio, and Hyper
        </p>
    </div>
</body>
</html>"#;

    Response {
        status: 200,
        body: Bytes::from(html),
        content_type: "text/html; charset=utf-8",
        headers: Vec::new(),
    }
}
