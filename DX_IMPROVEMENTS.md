# Developer Experience Improvements

This document summarizes the DX improvements added to the framework.

## 1. Type-Safe Parameter Extraction

**Problem:** Manual parameter parsing was verbose and error-prone:
```rust
let id: usize = match params.get("id") {
    Some(id_str) => match id_str.parse() {
        Ok(n) => n,
        Err(_) => return Response::json(&error, 400),
    },
    None => return Response::json(&error, 400),
};
```

**Solution:** `extract_param<T>` helper function:
```rust
use crate::engine::extract_param;

let id = match extract_param::<usize>(params, "id") {
    Ok(id) => id,
    Err(e) => return Response::json(&serde_json::json!({"error": e}), 400),
};
```

**Benefits:**
- Single line extraction with automatic type conversion
- Helpful error messages (e.g., "Missing path parameter: id", "Invalid id parameter: not a number")
- Works with any type implementing `FromStr`

**Location:** `core/src/engine/extractors.rs`

---

## 2. AppError Type for Error Handling

**Problem:** Manual error construction everywhere:
```rust
return Response::json(&serde_json::json!({
    "error": "Invalid JSON body",
    "details": e.to_string()
}), 400);
```

**Solution:** `AppError` type with automatic conversion:
```rust
use crate::engine::AppError;

fn handle_request(body: &Bytes) -> Result<User, AppError> {
    let data = Json::<CreateUser>::from_bytes(body)
        .map_err(|e| AppError::bad_request(format!("Invalid JSON: {}", e)))?;
    
    if data.0.name.is_empty() {
        return Err(AppError::bad_request("Name cannot be empty"));
    }
    
    Ok(user)
}

// In handler:
match handle_request(body) {
    Ok(user) => Response::json(&user, 201),
    Err(err) => err.into_response(),
}
```

**Benefits:**
- Result-based error handling with `?` operator
- Semantic error constructors (`bad_request`, `unauthorized`, `not_found`, etc.)
- Automatic conversion to JSON responses
- From implementations for common error types

**Location:** `core/src/engine/error.rs`

---

## 3. Improved WebSocket API

**Problem:** WebSocket setup required ~30 lines of boilerplate:
```rust
if !is_websocket_upgrade(req) {
    return Response::json(&error, 400);
}
let (response, ws_connection) = match upgrade_websocket(req) {
    Ok(result) => result,
    Err(e) => return Response::json(&error, 500),
};
tokio::spawn(async move {
    ws_connection.handle(|mut ws| async move {
        // handler logic
    }).await
});
// Convert Hyper response to framework Response...
```

**Solution:** Cleaner WebSocket connection handling:
```rust
use crate::engine::{is_websocket_upgrade, upgrade_websocket};

if !is_websocket_upgrade(req) {
    return Response::json(&serde_json::json!({"error": "WebSocket upgrade required"}), 400);
}

let (response, ws_connection) = upgrade_websocket(req)?;

tokio::spawn(async move {
    ws_connection.handle(|mut ws| async move {
        ws.send("Hello!").await?;
        while let Ok(Some(msg)) = ws.receive().await {
            // Handle messages
        }
        Ok(())
    }).await
});

// Return response...
```

**Note:** WebSocket routes still require some manual setup due to framework signature constraints. See `example/ws/chat.rs` for a complete working example.

**Location:** `core/src/engine/ws.rs`

---

## 4. Helper Functions for Common Patterns

Added middleware and response helper functions:

```rust
use crate::engine::helpers::{with_middleware, result_to_response};

// Simplified middleware application
let response = with_middleware(
    "GET".to_string(),
    "/endpoint".to_string(),
    &MY_MIDDLEWARE,
    || async { Response::json(&data, 200) }
).await;
```

**Location:** `core/src/engine/helpers.rs`

---

## Examples

See these files for complete working examples:

1. **Parameter Extraction:** `example/improved_dx/[id].rs`
2. **Error Handling:** `example/improved_dx.rs`
3. **WebSocket:** `example/ws/chat.rs`

---

## Performance Impact

All DX improvements maintain the framework's performance characteristics:
- Sub-millisecond response times maintained
- Zero-allocation route matching unchanged
- Helper functions compile to inline code with no overhead

## Migration Guide

### For existing routes:

1. **Replace manual parameter parsing:**
   ```rust
   // Before
   let id = params.get("id").unwrap().parse::<usize>().unwrap();
   
   // After
   let id = extract_param::<usize>(params, "id")?;
   ```

2. **Use AppError for validation:**
   ```rust
   // Before
   if name.is_empty() {
       return Response::json(&error_json, 400);
   }
   
   // After
   if name.is_empty() {
       return Err(AppError::bad_request("Name required"));
   }
   ```

3. **Simplify JSON parsing:**
   ```rust
   // Before
   let data = match Json::<T>::from_bytes(body) {
       Ok(json) => json,
       Err(e) => return Response::json(&error, 400),
   };
   
   // After
   let data = Json::<T>::from_bytes(body)
       .map_err(|e| AppError::bad_request(format!("Invalid JSON: {}", e)))?;
   ```
