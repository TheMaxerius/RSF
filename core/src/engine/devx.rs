// Developer experience utilities and helpers
use bytes::Bytes;
use serde::Serialize;
use std::collections::HashMap;

/// Request context with helpful methods
#[derive(Debug, Clone)]
pub struct RequestContext {
    pub method: String,
    pub path: String,
    pub params: HashMap<String, String>,
    pub query: HashMap<String, String>,
    pub headers: HashMap<String, String>,
}

impl RequestContext {
    pub fn new(method: String, path: String) -> Self {
        Self {
            method,
            path,
            params: HashMap::new(),
            query: HashMap::new(),
            headers: HashMap::new(),
        }
    }

    /// Get a path parameter by name
    #[inline]
    pub fn param(&self, name: &str) -> Option<&String> {
        self.params.get(name)
    }

    /// Get a query parameter by name
    #[inline]
    pub fn query(&self, name: &str) -> Option<&String> {
        self.query.get(name)
    }

    /// Get a header by name (case-insensitive)
    #[inline]
    pub fn header(&self, name: &str) -> Option<&String> {
        self.headers.get(&name.to_lowercase())
    }
}

/// Response builder with fluent API
#[derive(Debug)]
pub struct ResponseBuilder {
    status: u16,
    body: Option<Bytes>,
    content_type: &'static str,
    headers: Vec<(String, String)>,
}

impl ResponseBuilder {
    #[inline]
    pub fn new() -> Self {
        Self {
            status: 200,
            body: None,
            content_type: "text/plain; charset=utf-8",
            headers: Vec::new(),
        }
    }

    #[inline]
    pub fn status(mut self, code: u16) -> Self {
        self.status = code;
        self
    }

    #[inline]
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.body = Some(Bytes::from(text.into().into_bytes()));
        self.content_type = "text/plain; charset=utf-8";
        self
    }

    #[inline]
    pub fn html(mut self, html: impl Into<String>) -> Self {
        self.body = Some(Bytes::from(html.into().into_bytes()));
        self.content_type = "text/html; charset=utf-8";
        self
    }

    #[inline]
    pub fn json<T: Serialize>(mut self, data: &T) -> Self {
        if let Ok(json) = serde_json::to_vec(data) {
            self.body = Some(Bytes::from(json));
            self.content_type = "application/json; charset=utf-8";
        }
        self
    }

    #[inline]
    pub fn bytes(mut self, data: Bytes) -> Self {
        self.body = Some(data);
        self
    }

    #[inline]
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((key.into(), value.into()));
        self
    }

    #[inline]
    pub fn build(self) -> super::Response {
        super::Response {
            status: self.status,
            body: self.body.unwrap_or_else(|| Bytes::from_static(b"")),
            content_type: self.content_type,
            headers: self.headers,
        }
    }
}

impl Default for ResponseBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Quick response helpers
pub mod responses {
    use super::*;

    #[inline]
    pub fn ok(text: impl Into<String>) -> super::super::Response {
        ResponseBuilder::new().text(text).build()
    }

    #[inline]
    pub fn json<T: Serialize>(data: &T) -> super::super::Response {
        ResponseBuilder::new().json(data).build()
    }

    #[inline]
    pub fn html(content: impl Into<String>) -> super::super::Response {
        ResponseBuilder::new().html(content).build()
    }

    #[inline]
    pub fn not_found() -> super::super::Response {
        ResponseBuilder::new()
            .status(404)
            .text("Not Found")
            .build()
    }

    #[inline]
    pub fn internal_error(msg: impl Into<String>) -> super::super::Response {
        ResponseBuilder::new()
            .status(500)
            .text(msg)
            .build()
    }

    #[inline]
    pub fn redirect(location: impl Into<String>) -> super::super::Response {
        ResponseBuilder::new()
            .status(302)
            .header("Location", location)
            .build()
    }
}
