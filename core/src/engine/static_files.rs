/// Static file serving with caching and range support
use bytes::Bytes;
use std::path::{Path, PathBuf};
use tokio::fs;
use dashmap::DashMap;
use std::sync::Arc;

/// Static file server
pub struct StaticFileServer {
    root_dir: PathBuf,
    cache: Arc<DashMap<String, Bytes>>,
    cache_enabled: bool,
}

impl StaticFileServer {
    pub fn new(root_dir: impl Into<PathBuf>) -> Self {
        Self {
            root_dir: root_dir.into(),
            cache: Arc::new(DashMap::new()),
            cache_enabled: true,
        }
    }
    
    pub fn disable_cache(mut self) -> Self {
        self.cache_enabled = false;
        self
    }
    
    /// Serve a file by path
    pub async fn serve(&self, path: &str) -> Result<StaticFile, String> {
        // Security: prevent directory traversal
        let safe_path = self.sanitize_path(path)?;
        let full_path = self.root_dir.join(&safe_path);
        
        // Check if path exists and is a file
        if !full_path.exists() {
            return Err("File not found".to_string());
        }
        
        if !full_path.is_file() {
            return Err("Not a file".to_string());
        }
        
        // Check cache first
        if self.cache_enabled {
            if let Some(cached) = self.cache.get(path) {
                return Ok(StaticFile {
                    content: cached.clone(),
                    content_type: Self::guess_content_type(&full_path),
                    etag: None,
                });
            }
        }
        
        // Read file
        let content = fs::read(&full_path).await
            .map_err(|e| format!("Failed to read file: {}", e))?;
        
        let bytes = Bytes::from(content);
        
        // Cache if enabled
        if self.cache_enabled {
            self.cache.insert(path.to_string(), bytes.clone());
        }
        
        Ok(StaticFile {
            content: bytes,
            content_type: Self::guess_content_type(&full_path),
            etag: None,
        })
    }
    
    /// Sanitize path to prevent directory traversal
    fn sanitize_path(&self, path: &str) -> Result<String, String> {
        let path = path.trim_start_matches('/');
        
        // Check for directory traversal attempts
        if path.contains("..") {
            return Err("Invalid path: directory traversal detected".to_string());
        }
        
        Ok(path.to_string())
    }
    
    /// Guess content type from file extension
    fn guess_content_type(path: &Path) -> &'static str {
        match path.extension().and_then(|e| e.to_str()) {
            Some("html") | Some("htm") => "text/html; charset=utf-8",
            Some("css") => "text/css; charset=utf-8",
            Some("js") => "application/javascript; charset=utf-8",
            Some("json") => "application/json",
            Some("xml") => "application/xml",
            Some("png") => "image/png",
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("gif") => "image/gif",
            Some("svg") => "image/svg+xml",
            Some("ico") => "image/x-icon",
            Some("pdf") => "application/pdf",
            Some("zip") => "application/zip",
            Some("txt") => "text/plain; charset=utf-8",
            Some("wasm") => "application/wasm",
            Some("mp4") => "video/mp4",
            Some("webm") => "video/webm",
            Some("mp3") => "audio/mpeg",
            Some("woff") => "font/woff",
            Some("woff2") => "font/woff2",
            Some("ttf") => "font/ttf",
            _ => "application/octet-stream",
        }
    }
    
    /// Clear cache
    pub fn clear_cache(&self) {
        self.cache.clear();
    }
}

impl Clone for StaticFileServer {
    fn clone(&self) -> Self {
        Self {
            root_dir: self.root_dir.clone(),
            cache: Arc::clone(&self.cache),
            cache_enabled: self.cache_enabled,
        }
    }
}

/// Static file response
pub struct StaticFile {
    pub content: Bytes,
    pub content_type: &'static str,
    pub etag: Option<String>,
}

impl StaticFile {
    pub fn to_response(self) -> crate::engine::Response {
        let mut headers = Vec::new();
        if let Some(etag) = self.etag {
            headers.push(("ETag".to_string(), etag));
        }
        
        crate::engine::Response {
            status: 200,
            body: self.content,
            content_type: self.content_type,
            headers,
        }
    }
}
