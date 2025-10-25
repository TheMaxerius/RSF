use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;

// Mock the route matching (we'll compile this separately)
fn bench_route_matching(c: &mut Criterion) {
    c.bench_function("route_match_static", |b| {
        b.iter(|| {
            // Simulates matching /api
            let route = black_box("/api");
            let method = black_box("GET");
            
            // Optimized path
            let method_bytes = method.as_bytes();
            let route_normalized = route.trim_start_matches('/').trim_end_matches('/');
            let seg_count = if route_normalized.is_empty() { 0 } else { route_normalized.bytes().filter(|&b| b == b'/').count() + 1 };
            
            let mut seg_buf: [&str; 8] = [""; 8];
            let segments = if seg_count <= 8 {
                let mut i = 0;
                for seg in route_normalized.split('/') {
                    if i >= 8 { break; }
                    seg_buf[i] = seg;
                    i += 1;
                }
                &seg_buf[..seg_count]
            } else {
                &[][..]
            };
            
            // Match check
            let matched = method_bytes.len() == 3 && method_bytes == b"GET" && seg_count == 1 && segments[0] == "api";
            black_box(matched)
        })
    });
    
    c.bench_function("route_match_dynamic", |b| {
        b.iter(|| {
            // Simulates matching /users/123
            let route = black_box("/users/123");
            let method = black_box("GET");
            
            let method_bytes = method.as_bytes();
            let route_normalized = route.trim_start_matches('/').trim_end_matches('/');
            let seg_count = if route_normalized.is_empty() { 0 } else { route_normalized.bytes().filter(|&b| b == b'/').count() + 1 };
            
            let mut seg_buf: [&str; 8] = [""; 8];
            let segments = if seg_count <= 8 {
                let mut i = 0;
                for seg in route_normalized.split('/') {
                    if i >= 8 { break; }
                    seg_buf[i] = seg;
                    i += 1;
                }
                &seg_buf[..seg_count]
            } else {
                &[][..]
            };
            
            // Match check with param extraction
            if method_bytes.len() == 3 && method_bytes == b"GET" && seg_count == 2 && segments[0] == "users" {
                let mut params = HashMap::with_capacity(1);
                params.insert("id".to_string(), segments[1].to_string());
                black_box(params)
            } else {
                black_box(HashMap::new())
            }
        })
    });
    
    c.bench_function("route_match_nested", |b| {
        b.iter(|| {
            // Simulates matching /posts/42/comments/7
            let route = black_box("/posts/42/comments/7");
            let method = black_box("GET");
            
            let method_bytes = method.as_bytes();
            let route_normalized = route.trim_start_matches('/').trim_end_matches('/');
            let seg_count = if route_normalized.is_empty() { 0 } else { route_normalized.bytes().filter(|&b| b == b'/').count() + 1 };
            
            let mut seg_buf: [&str; 8] = [""; 8];
            let segments = if seg_count <= 8 {
                let mut i = 0;
                for seg in route_normalized.split('/') {
                    if i >= 8 { break; }
                    seg_buf[i] = seg;
                    i += 1;
                }
                &seg_buf[..seg_count]
            } else {
                &[][..]
            };
            
            // Match check with param extraction
            if method_bytes.len() == 3 && method_bytes == b"GET" && seg_count == 4 
                && segments[0] == "posts" && segments[2] == "comments" {
                let mut params = HashMap::with_capacity(2);
                params.insert("id".to_string(), segments[1].to_string());
                params.insert("commentId".to_string(), segments[3].to_string());
                black_box(params)
            } else {
                black_box(HashMap::new())
            }
        })
    });
}

criterion_group!(benches, bench_route_matching);
criterion_main!(benches);
