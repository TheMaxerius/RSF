// handle http requests and route them to the runtime
use crate::engine::runtime::Runtime;
use crate::engine::parser::{ProjectFile, RouteSegment};
use tokio::fs;
use std::collections::HashMap;
use std::sync::Arc;
use dashmap::DashMap;
use percent_encoding::percent_decode_str;
use bytes::Bytes;
use ahash::AHashMap;
use smallvec::SmallVec;
use once_cell::sync::Lazy;

// Type alias for route params - uses SmallVec for stack allocation when <= 4 params
type RouteParams = SmallVec<[(String, String); 4]>;

// Common path strings to avoid allocations
static HEALTH_PATH: &str = "/health";
static ROOT_PATH: &str = "/";

// Include the compile-time generated router (created by build.rs)
include!("generated_routes.rs");

// Static responses to avoid allocations
static HEALTH_RESPONSE: Lazy<super::Response> = Lazy::new(|| super::Response {
    status: 200,
    body: Bytes::from_static(b"OK"),
    content_type: "text/plain; charset=utf-8",
    headers: Vec::new(),
});

static NOT_FOUND_RESPONSE: Lazy<super::Response> = Lazy::new(|| super::Response {
    status: 404,
    body: Bytes::from_static(b"Not Found"),
    content_type: "text/plain; charset=utf-8",
    headers: Vec::new(),
});

static BAD_REQUEST_RESPONSE: Lazy<super::Response> = Lazy::new(|| super::Response {
    status: 400,
    body: Bytes::from_static(b"Bad Request"),
    content_type: "text/plain; charset=utf-8",
    headers: Vec::new(),
});

static INTERNAL_ERROR_RESPONSE: Lazy<super::Response> = Lazy::new(|| super::Response {
    status: 500,
    body: Bytes::from_static(b"Internal Server Error"),
    content_type: "text/plain; charset=utf-8",
    headers: Vec::new(),
});

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

impl Response {
    pub fn json<T: serde::Serialize>(data: &T, status: u16) -> Self {
        let json_str = serde_json::to_string(data).unwrap_or_else(|_| "{}".to_string());
        Response {
            status,
            body: Bytes::from(json_str),
            content_type: "application/json; charset=utf-8",
            headers: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RequestHandler {
    pub runtime: Arc<Runtime>,
    /// Simple in-memory route lookup cache: (method,path) -> file_path (Arc to avoid cloning large strings)
    pub route_cache: Arc<DashMap<String, Option<Arc<String>>>>,
    /// Cache matched params for a (method,path) so we don't re-run match_route on cache hits.
    /// Using SmallVec for better cache locality and stack allocation
    pub route_params_cache: Arc<DashMap<String, Option<Arc<RouteParams>>>>,
}

impl RequestHandler {
    pub fn new(runtime: &Runtime) -> Self {
        RequestHandler {
            runtime: Arc::new(runtime.clone()),
            route_cache: Arc::new(DashMap::new()),
            route_params_cache: Arc::new(DashMap::new()),
        }
    }

    /// Handle a request with logging and timing
    pub async fn handle_request_with_logging(&self, method: &str, raw_path: &str, body: Bytes) -> super::Response {
        let start = std::time::Instant::now();
        let response = self.handle_request(method, raw_path, body).await;
        let duration = start.elapsed();
        
        // Log request with response time
        let duration_ms = duration.as_micros() as f64 / 1000.0;
        let status_color = match response.status {
            200..=299 => "\x1b[32m", // Green
            300..=399 => "\x1b[36m", // Cyan
            400..=499 => "\x1b[33m", // Yellow
            _ => "\x1b[31m",         // Red
        };
        
        log::info!(
            "{}{}\x1b[0m {} {} - {:.2}ms",
            status_color,
            response.status,
            method,
            raw_path,
            duration_ms
        );
        
        response
    }

    /// Handle a request asynchronously and return a raw HTTP response string.
    #[inline]
    async fn handle_request(&self, method: &str, raw_path: &str, body: Bytes) -> super::Response {
        let method = method.to_string();

        // Basic sanitization and decode path
        let path = sanitize_and_decode_path(raw_path);

        // Try compile-time generated router first - now returns (handler, params)
        if let Some((h, params)) = get_handler(&path, &method) {
            // Use the extracted params from the router directly, passing body as well
            let resp = h(&params, &body).await;
            return resp;
        }

        // fallback: serve registered files directly (useful during development)
        if method.eq_ignore_ascii_case("GET") {
            for file in &self.runtime.project_files {
                if let Some(params) = match_route_fast(file, &path) {
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
            status: NOT_FOUND_RESPONSE.status,
            body: NOT_FOUND_RESPONSE.body.clone(),
            content_type: NOT_FOUND_RESPONSE.content_type,
            headers: Vec::new(),
        }
    }
}

/// Match a request path against precomputed route segments from a ProjectFile.
/// Returns SmallVec of params for better cache locality. Optimized with precomputed segments.
#[inline(always)]
fn match_route_fast(file: &crate::engine::parser::ProjectFile, req_path: &str) -> Option<RouteParams> {
    use crate::engine::parser::RouteSegment;
    
    // strip query string
    let req = req_path.split('?').next().unwrap_or("").trim();
    let req = if req == "/" {
        ""
    } else {
        req.trim_start_matches('/').trim_end_matches('/')
    };

    let req_segments: Vec<&str> = if req.is_empty() {
        Vec::new()
    } else {
        req.split('/').collect()
    };

    if file.route_segments.len() != req_segments.len() {
        return None;
    }

    // Use SmallVec for stack allocation when <= 4 params
    let mut params = RouteParams::new();
    for (route_seg, rseg) in file.route_segments.iter().zip(req_segments.iter()) {
        match route_seg {
            RouteSegment::Dynamic(name) => {
                // decode percent-encoding in rseg
                let decoded = percent_decode_str(rseg).decode_utf8_lossy().to_string();
                params.push((name.clone(), decoded));
            }
            RouteSegment::Static(expected) => {
                let decoded = percent_decode_str(rseg).decode_utf8_lossy();
                if expected != &*decoded {
                    return None;
                }
            }
        }
    }

    Some(params)
}

/// Legacy match_route for backward compatibility (uses file_path string)
#[inline]
fn match_route(file_fp: &str, req_path: &str) -> Option<RouteParams> {
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

    let mut params = RouteParams::new();
    for (fseg, rseg) in fp_segments.iter().zip(req_segments.iter()) {
        // decode percent-encoding in rseg
        let decoded = percent_decode_str(rseg).decode_utf8_lossy().to_string();
        if fseg.starts_with('[') && fseg.ends_with(']') {
            let name = &fseg[1..fseg.len() - 1];
            params.push((name.to_string(), decoded));
            continue;
        }
        if fseg != &decoded {
            return None;
        }
    }

    Some(params)
}

#[inline(always)]
fn sanitize_and_decode_path(p: &str) -> String {
    // remove any trailing/leading whitespace, disallow \0, and decode percent-encoding
    let p = p.trim().split('\0').next().unwrap_or("");
    // Only keep path component before query
    let before_q = p.split('?').next().unwrap_or("");
    let decoded = percent_decode_str(before_q).decode_utf8_lossy();
    
    // collapse // and remove .. segments for basic traversal protection
    // Preallocate capacity based on decoded length
    let mut parts = Vec::with_capacity(8); // typical depth
    for seg in decoded.split('/') {
        if seg.is_empty() || seg == "." {
            continue;
        }
        if seg == ".." {
            parts.pop();
            continue;
        }
        parts.push(seg);
    }
    
    if parts.is_empty() {
        return "/".to_string();
    }
    
    // Preallocate string capacity to avoid reallocs
    let total_len: usize = parts.iter().map(|s| s.len()).sum();
    let mut result = String::with_capacity(total_len + parts.len() + 1);
    for part in parts {
        result.push('/');
        result.push_str(part);
    }
    result
}

#[inline(always)]
fn content_type_for_path(path: &str) -> &'static str {
    // Optimized: check last few chars to avoid full string scan
    let bytes = path.as_bytes();
    let len = bytes.len();
    
    if len >= 5 {
        match &bytes[len-5..] {
            b".html" => return "text/html; charset=utf-8",
            b".json" => return "application/json; charset=utf-8",
            b".jpeg" => return "image/jpeg",
            _ => {}
        }
    }
    
    if len >= 4 {
        match &bytes[len-4..] {
            b".htm" => return "text/html; charset=utf-8",
            b".css" => return "text/css; charset=utf-8",
            b".svg" => return "image/svg+xml",
            b".png" => return "image/png",
            b".jpg" => return "image/jpeg",
            _ => {}
        }
    }
    
    if len >= 3 && &bytes[len-3..] == b".js" {
        return "application/javascript; charset=utf-8";
    }
    
    "text/plain; charset=utf-8"
}

#[inline]
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