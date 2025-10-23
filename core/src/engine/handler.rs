// handle http requests and route them to the runtime
use crate::engine::runtime::Runtime;
use tokio::fs;
use std::collections::HashMap;
use std::sync::Arc;
use dashmap::DashMap;
use percent_encoding::percent_decode_str;
use bytes::Bytes;

// Include the compile-time generated router (created by build.rs)
include!("generated_routes.rs");

pub struct Request {
    pub path: String,
    pub method: String,
    pub body: Option<String>,
}

pub struct Response {
    pub status: u16,
    pub body: Bytes,
    pub content_type: &'static str,
    pub headers: Vec<(String, String)>,
}

#[derive(Clone, Debug)]
pub struct RequestHandler {
    pub runtime: Arc<Runtime>,
    /// Simple in-memory route lookup cache: (method,path) -> file_path
    pub route_cache: Arc<DashMap<String, Option<String>>>,
}

impl RequestHandler {
    pub fn new(runtime: &Runtime) -> Self {
        RequestHandler {
            runtime: Arc::new(runtime.clone()),
            route_cache: Arc::new(DashMap::new()),
        }
    }

    /// Handle a request asynchronously and return a raw HTTP response string.
    pub async fn handle_request(&self, method: &str, raw_path: &str) -> super::Response {
        let method = method.to_string();

        // Basic sanitization and decode path
        let path = sanitize_and_decode_path(raw_path);
        if method.eq_ignore_ascii_case("GET") && path == "/health" {
            return super::Response {
                status: 200,
                body: Bytes::from_static(b"OK"),
                content_type: "text/plain; charset=utf-8",
                headers: Vec::new(),
            };
        }

        // Try compile-time generated router first
        if let Some(h) = get_handler(&path, &method) {
            // Attempt to find matching project file quickly using the cache
            let cache_key = format!("{}:{}", method, path);
            if let Some(entry) = self.route_cache.get(&cache_key) {
                if let Some(file_path) = entry.value().clone() {
                    if let Some(params) = match_route(&file_path, &path) {
                        let resp = h(&params);
                        return resp;
                    }
                }
            }

            // not in cache: search project files and populate cache
            for file in &self.runtime.project_files {
                if let Some(params) = match_route(&file.file_path, &path) {
                    // cache positive hit
                    self.route_cache.insert(cache_key.clone(), Some(file.file_path.clone()));
                    // call the handler function with params
                    let resp = h(&params);
                    return resp;
                }
            }

            // cache negative result to avoid repeated work
            self.route_cache.insert(cache_key, None);
        }

        // fallback: serve registered files directly (useful during development)
        if method.eq_ignore_ascii_case("GET") {
            for file in &self.runtime.project_files {
                if let Some(params) = match_route(&file.file_path, &path) {
                    // Security: prefer cached contents in production
                    if !self.runtime.dev {
                        if let Some(cached) = self.runtime.file_cache.get(&file.full_path) {
                            let contents = cached.clone();
                            if params.is_empty() {
                                return super::Response {
                                    status: 200,
                                    body: contents,
                                    content_type: content_type_for_path(&file.full_path),
                                    headers: Vec::new(),
                                };
                            } else {
                                // need to prepend params text
                                let mut v = Vec::new();
                                let mut params_text = String::from("\nParams:\n");
                                for (k, v_) in &params {
                                    params_text.push_str(&format!("{} = {}\n", k, v_));
                                }
                                v.extend_from_slice(params_text.as_bytes());
                                v.extend_from_slice(&contents);
                                return super::Response {
                                    status: 200,
                                    body: Bytes::from(v),
                                    content_type: content_type_for_path(&file.full_path),
                                    headers: Vec::new(),
                                };
                            }
                        }
                    }

                    // fallback: async read from filesystem (dev mode or cache miss)
                    match fs::read(&file.full_path).await {
                        Ok(contents_vec) => {
                            if params.is_empty() {
                                return super::Response {
                                    status: 200,
                                    body: Bytes::from(contents_vec),
                                    content_type: content_type_for_path(&file.full_path),
                                    headers: Vec::new(),
                                };
                            } else {
                                let mut v = Vec::new();
                                let mut params_text = String::from("\nParams:\n");
                                for (k, v_) in &params {
                                    params_text.push_str(&format!("{} = {}\n", k, v_));
                                }
                                v.extend_from_slice(params_text.as_bytes());
                                v.extend_from_slice(&contents_vec);
                                return super::Response {
                                    status: 200,
                                    body: Bytes::from(v),
                                    content_type: content_type_for_path(&file.full_path),
                                    headers: Vec::new(),
                                };
                            }
                        }
                        Err(e) => {
                            let body = format!("Failed to read file: {}", e);
                            return super::Response {
                                status: 500,
                                body: Bytes::from(body.into_bytes()),
                                content_type: "text/plain; charset=utf-8",
                                headers: Vec::new(),
                            };
                        }
                    }
                }
            }
        }

        super::Response {
            status: 404,
            body: Bytes::from_static(b"Not Found"),
            content_type: "text/plain; charset=utf-8",
            headers: Vec::new(),
        }
    }
}

/// Match a request path against a project file path which may include Next.js-style
/// dynamic segments like `users/[id].rs` or `admin/index.rs`.
/// Returns a map of param name -> value when matched.
fn match_route(file_fp: &str, req_path: &str) -> Option<HashMap<String, String>> {
    // strip query string
    let req = req_path.split('?').next().unwrap_or("").trim();
    let req = if req == "/" {
        ""
    } else {
        req.trim_start_matches('/').trim_end_matches('/')
    };

    // Normalize file path to route segments
    let fp = file_fp.trim();
    // handle index.rs specially
    let route_fp = if fp.ends_with("index.rs") {
        let dir = fp.trim_end_matches("index.rs").trim_end_matches('/');
        dir.trim_end_matches('/')
    } else {
        fp.trim_end_matches(".rs")
    };

    let fp_segments: Vec<&str> = if route_fp.is_empty() {
        Vec::new()
    } else {
        route_fp.split('/').collect()
    };
    let req_segments: Vec<&str> = if req.is_empty() {
        Vec::new()
    } else {
        req.split('/').collect()
    };

    if fp_segments.len() != req_segments.len() {
        return None;
    }

    let mut params = HashMap::new();
    for (fseg, rseg) in fp_segments.iter().zip(req_segments.iter()) {
        // decode percent-encoding in rseg
        let decoded = percent_decode_str(rseg).decode_utf8_lossy().to_string();
        if fseg.starts_with('[') && fseg.ends_with(']') {
            let name = &fseg[1..fseg.len() - 1];
            params.insert(name.to_string(), decoded);
            continue;
        }
        if fseg != &decoded {
            return None;
        }
    }

    Some(params)
}

fn sanitize_and_decode_path(p: &str) -> String {
    // remove any trailing/leading whitespace, disallow \0, and decode percent-encoding
    let p = p.trim().split('\0').next().unwrap_or("");
    // Only keep path component before query
    let before_q = p.split('?').next().unwrap_or("");
    let decoded = percent_decode_str(before_q).decode_utf8_lossy().to_string();
    // collapse // and remove .. segments for basic traversal protection
    let mut parts = Vec::new();
    for seg in decoded.split('/') {
        if seg == "" || seg == "." {
            continue;
        }
        if seg == ".." {
            parts.pop();
            continue;
        }
        parts.push(seg);
    }
    let s = format!("/{}", parts.join("/"));
    if s == "/" {
        "/".to_string()
    } else {
        s
    }
}

fn content_type_for_path(path: &str) -> &'static str {
    if path.ends_with(".html") || path.ends_with(".htm") {
        "text/html; charset=utf-8"
    } else if path.ends_with(".js") {
        "application/javascript; charset=utf-8"
    } else if path.ends_with(".css") {
        "text/css; charset=utf-8"
    } else if path.ends_with(".json") {
        "application/json; charset=utf-8"
    } else if path.ends_with(".svg") {
        "image/svg+xml"
    } else if path.ends_with(".png") {
        "image/png"
    } else if path.ends_with(".jpg") || path.ends_with(".jpeg") {
        "image/jpeg"
    } else {
        "text/plain; charset=utf-8"
    }
}

pub(crate) fn status_text(code: u16) -> &'static str {
    match code {
        200 => "OK",
        201 => "Created",
        204 => "No Content",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        500 => "Internal Server Error",
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        _ => "",
    }
}