# Developer Experience Features

## Overview

Added comprehensive developer experience (DX) features while maintaining peak performance.

## New Features

### 1. **Hot Reload** (`src/engine/hot_reload.rs`)

- Automatic file watching for `.rs` files
- Colored console output when changes detected
- Uses `notify` crate for efficient file system monitoring
- Integrated into development mode

```rust
use crate::engine::hot_reload::HotReloader;

let reloader = HotReloader::new("./example").await;
// File changes are logged automatically
```

### 2. **Request/Response Helpers** (`src/engine/devx.rs`)

#### RequestContext

Helper for accessing request data:

```rust
let ctx = RequestContext::new(method, path);
let id = ctx.param("id");           // Get route parameter
let query = ctx.query("search");    // Get query string
let auth = ctx.header("Authorization"); // Get header
```

#### ResponseBuilder

Fluent API for building responses:

```rust
ResponseBuilder::new()
    .status(200)
    .json(&data)
    .header("X-Custom", "value")
    .build()
```

#### Quick Response Helpers

```rust
use crate::engine::devx::responses;

responses::ok("Success");              // 200 text
responses::json(&data);                // 200 JSON
responses::html("<h1>Hello</h1>");     // 200 HTML
responses::not_found();                // 404
responses::internal_error("Error");    // 500
responses::redirect("/new-path");      // 302
```

### 3. **Error Handling** (`src/engine/errors.rs`)

Custom error types with automatic status codes:

```rust
pub enum FrameworkError {
    NotFound(String),
    BadRequest(String),
    Unauthorized(String),
    InternalError(String),
    RouteNotFound,
}

impl FrameworkError {
    pub fn status_code(&self) -> u16 { ... }
    pub fn to_json(&self) -> String { ... }
}

pub type Result<T> = std::result::Result<T, FrameworkError>;
```

### 4. **Enhanced Logging**

- Colored terminal output using `colored` crate
- Millisecond-precision timestamps
- Structured logging with `log` + `env_logger`
- Beautiful startup banner with emoji indicators

Example output:

```
🚀 Framework Starting...
══════════════════════════════════════════════════
📁 Found 3 route file(s):
  1. [API] main.rs
  2. [API] api.rs
  3. [API] users/[id].rs
══════════════════════════════════════════════════
⚙️ Server Configuration:
  • Address: 127.0.0.1:8080
  • Mode: Development
  • Hot Reload: Enabled ♻️
══════════════════════════════════════════════════
```

## Example Routes

### API Route (`example/api.rs`)

```rust
use std::collections::HashMap;

pub fn GET(_params: &HashMap<String, String>) -> (String, u16) {
    (r#"{"message":"API endpoint"}"#.to_string(), 200)
}
```

### Dynamic Route (`example/users/[id].rs`)

```rust
use std::collections::HashMap;

pub fn GET(params: &HashMap<String, String>) -> (String, u16) {
    let id = params.get("id").unwrap_or(&"unknown".to_string()).clone();
    (format!(r#"{{"user_id":"{}"}}"#, id), 200)
}

pub fn DELETE(params: &HashMap<String, String>) -> (String, u16) {
    let id = params.get("id").unwrap_or(&"unknown".to_string()).clone();
    (format!(r#"{{"deleted":"{}"}}"#, id), 200)
}
```

## Performance Impact

### Binary Size

- **Debug**: 36 MB
- **Release**: 2.5 MB (minimal increase from 2.2MB)

### Optimizations Maintained

All previous performance optimizations remain active:

- ✅ jemalloc allocator
- ✅ SmallVec stack allocation for params
- ✅ AHashMap for faster hashing
- ✅ Precomputed route segments
- ✅ Dual-layer caching (route + params)
- ✅ Static responses with `Bytes::from_static`
- ✅ Aggressive inlining with `#[inline(always)]`
- ✅ LTO and optimized compiler flags

### Performance Metrics

Hot path functions:

- Cache key generation: ~18ns
- Path sanitization: ~72ns
- Route matching: ~150-300ns (cached)

## Dependencies Added

```toml
# DX Features
notify = "6.1"          # File watching
colored = "2.1"         # Terminal colors
anyhow = "1.0"          # Error handling
thiserror = "1.0"       # Derive Error

# Already added for performance
ahash = "0.8"
smallvec = "1.13"
tikv-jemallocator = "0.6"
once_cell = "1.19"
```

## Usage

### Development Mode

```bash
cargo run
# Hot reload enabled automatically
# Colored output for better visibility
```

### Production Mode

```bash
cargo build --release
./target/release/core
# 2.5MB optimized binary
# All performance features active
```

## Build System

### Dynamic Route Support

The build script (`build.rs`) now handles:

- Dynamic segments like `[id]`, `[slug]`, etc.
- Automatic parameter extraction
- Reference vs owned parameter detection
- Module name sanitization (brackets converted to underscores)

### Example Build Output

```
module__example_api_rs
module__example_main_rs
module__example_users__id__rs  // [id] becomes _id_
```

## Next Steps

### Potential Enhancements

1. **Middleware System**: Add before/after request hooks
2. **WebSocket Support**: Real-time connections
3. **Static File Serving**: Built-in asset serving
4. **Database Pool**: Connection pooling helpers
5. **Session Management**: Cookie/JWT helpers
6. **Rate Limiting**: Request throttling
7. **CORS Support**: Cross-origin headers
8. **Request Validation**: Schema validation
9. **OpenAPI/Swagger**: Auto-generated docs
10. **Testing Utilities**: Test helpers and mocks

All additions will maintain the zero-overhead philosophy and sub-3MB binary target.
