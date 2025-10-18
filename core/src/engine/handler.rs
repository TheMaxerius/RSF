

// handle http requests and route them to the runtime
use crate::engine::runtime::Runtime;
use tokio::fs;
use std::collections::HashMap;
// Include the compile-time generated router (created by build.rs)
include!("generated_routes.rs");

pub struct Request {
    pub path: String,
    pub method: String,
    pub body: Option<String>,
}

pub struct Response {
    pub status: u16,
    pub body: String,
}

#[derive(Clone, Debug)]
pub struct RequestHandler {
    pub runtime: Runtime,
}

impl RequestHandler {
    pub fn new(runtime: &Runtime) -> Self {
        RequestHandler { runtime: runtime.clone() }
    }

    /// Handle a request asynchronously and return a raw HTTP response string.
    pub async fn handle_request(&self, method: &str, path: &str) -> String {
        if method.eq_ignore_ascii_case("GET") && path == "/health" {
            let body = "OK";
            return format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain; charset=utf-8\r\n\r\n{}",
                body.len(),
                body
            );
        }

        // Try compile-time generated router first
        if let Some(h) = get_handler(path, method) {
            // match_route to extract params into a HashMap
            for file in &self.runtime.project_files {
                if let Some(params) = match_route(&file.file_path, path) {
                    // call the handler function with params
                    let out = h(&params);
                    return format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain; charset=utf-8\r\n\r\n{}",
                        out.len(),
                        out
                    );
                }
            }
        }

        // fallback: serve registered files directly (useful during development)
        for file in &self.runtime.project_files {
            if method.eq_ignore_ascii_case("GET") {
                if let Some(params) = match_route(&file.file_path, path) {
                        // attempt to serve from in-memory cache first (populated in Runtime::new when not dev)
                        if !self.runtime.dev {
                            if let Some(cached) = self.runtime.file_cache.get(&file.full_path) {
                                let mut contents = cached.clone();
                                if !params.is_empty() {
                                    let mut params_text = String::from("\nParams:\n");
                                    for (k, v) in &params {
                                        params_text.push_str(&format!("{} = {}\n", k, v));
                                    }
                                    contents = format!("{}{}", params_text, contents);
                                }
                                return format!(
                                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain; charset=utf-8\r\n\r\n{}",
                                    contents.len(),
                                    contents
                                );
                            }
                        }

                        // fallback: async read from filesystem (dev mode or cache miss)
                        match fs::read_to_string(&file.full_path).await {
                            Ok(mut contents) => {
                                if !params.is_empty() {
                                    let mut params_text = String::from("\nParams:\n");
                                    for (k, v) in &params {
                                        params_text.push_str(&format!("{} = {}\n", k, v));
                                    }
                                    contents = format!("{}{}", params_text, contents);
                                }
                                return format!(
                                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain; charset=utf-8\r\n\r\n{}",
                                    contents.len(),
                                    contents
                                );
                            }
                            Err(e) => {
                                let body = format!("Failed to read file: {}", e);
                                return format!(
                                    "HTTP/1.1 500 Internal Server Error\r\nContent-Length: {}\r\nContent-Type: text/plain; charset=utf-8\r\n\r\n{}",
                                    body.len(),
                                    body
                                );
                            }
                        }
                }
            }
        }

        // default 404 response
        let body = "Not Found";
        format!(
            "HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\nContent-Type: text/plain; charset=utf-8\r\n\r\n{}",
            body.len(),
            body
        )
    }
}

/// Match a request path against a project file path which may include Next.js-style
/// dynamic segments like `users/[id].rs` or `admin/index.rs`.
/// Returns a map of param name -> value when matched.
fn match_route(file_fp: &str, req_path: &str) -> Option<HashMap<String, String>> {
    // Normalize request path (remove leading/trailing slashes)
    let req = req_path.trim();
    let req = if req == "/" { "" } else { req.trim_start_matches('/').trim_end_matches('/') };

    // Normalize file path to route segments
    let fp = file_fp.trim();
    // handle index.rs specially
    let route_fp = if fp.ends_with("index.rs") {
        let dir = fp.trim_end_matches("index.rs").trim_end_matches('/');
        dir.trim_end_matches('/')
    } else {
        fp.trim_end_matches(".rs")
    };

    let fp_segments: Vec<&str> = if route_fp.is_empty() { Vec::new() } else { route_fp.split('/').collect() };
    let req_segments: Vec<&str> = if req.is_empty() { Vec::new() } else { req.split('/').collect() };

    if fp_segments.len() != req_segments.len() {
        return None;
    }

    let mut params = HashMap::new();
    for (fseg, rseg) in fp_segments.iter().zip(req_segments.iter()) {
        if fseg.starts_with('[') && fseg.ends_with(']') {
            let name = &fseg[1..fseg.len()-1];
            params.insert(name.to_string(), rseg.to_string());
            continue;
        }
        if fseg != rseg {
            return None;
        }
    }

    Some(params)
}

