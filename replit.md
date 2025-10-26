# Overview

This is a high-performance Rust web framework featuring Next.js-style file-based routing with zero-overhead design principles. The framework prioritizes blazing-fast performance (sub-millisecond response times, 30-80ns route matching) while maintaining excellent developer experience through familiar routing patterns and async/await support. The architecture leverages compile-time code generation for route matching, jemalloc for memory optimization, and full async support via Tokio runtime.

# User Preferences

Preferred communication style: Simple, everyday language.

# System Architecture

## Core Framework Architecture

**File-Based Routing System**
- Routes are automatically generated from filesystem structure (e.g., `users/[id].rs` → `/users/:id`)
- Compile-time code generation produces zero-allocation, stack-based route matching
- Supports static routes, dynamic parameters, nested dynamic routes, and unlimited nesting depth
- Route matching achieves 30-80ns performance through compile-time optimizations

**Request Processing Pipeline**
- Async handler system using `Pin<Box<dyn Future>>` for concurrent request handling
- Type-safe extractors for parsing request bodies: `Json<T>`, `Form`, `Text`, `RawBody`
- Built-in URL decoding
- Support for all standard HTTP methods (GET, POST, PUT, DELETE, PATCH, OPTIONS, HEAD)

**Performance Optimizations**
- Full Link-Time Optimization (LTO) producing 2.5 MB release binaries
- jemalloc allocator replacing standard allocator for superior memory performance
- AHashMap providing 2x faster hashing than standard HashMap
- Zero heap allocations for route matching operations
- Response times consistently under 0.15ms (sub-millisecond)

## Runtime Architecture

**Async Runtime**
- Tokio-based async runtime for handling concurrent requests
- Full async/await support throughout the framework
- Non-blocking I/O operations

**Development Tooling**
- Hot reload capability during development (watches .rs files for changes)
- Colored terminal output with request logging
- CLI tool for project scaffolding and management (`cli/` crate)

## Project Structure

**Core Framework (`core/` crate)**
- Main web server implementation
- Route generation and matching engine
- Request/response handling
- Configuration stored in `core/src/engine/project.json` (project name, version, parent folder for route discovery)

**CLI Tool (`cli/` crate)**
- Interactive project creation (`new` command) - copies `core/` template or scaffolds minimal crate
- Development server runner (`dev` command) - runs `cargo run` in core crate
- Configuration editor (`edit` command) - updates project.json settings

**Example Routes (`example/` directory)**
- Sample route implementations demonstrating framework capabilities
- Parent folder path configured via project.json

## Design Philosophy

**Zero Configuration**
- Routes automatically discovered from file structure
- No manual route registration required
- Convention-over-configuration approach

**Type Safety**
- Compile-time route generation ensures type-safe routing
- Type-safe request body extractors prevent runtime errors
- Leverages Rust's type system for correctness guarantees

**Performance-First**
- All architectural decisions prioritize runtime performance
- Compile-time code generation over runtime reflection
- Stack allocation over heap allocation where possible
- Optimized memory allocator (jemalloc)

# External Dependencies

## Core Runtime Dependencies

**Tokio (async runtime)**
- Provides async/await runtime for concurrent request handling
- Used for non-blocking I/O operations
- Includes macros, networking, filesystem, and process support

**HTTP Stack**
- Hyper: Low-level HTTP implementation (client, server, HTTP/1, HTTP/2)
- Tower: Service abstractions and middleware
- HTTP body handling and parsing utilities

## Performance Libraries

**jemalloc**
- Custom memory allocator replacing system allocator
- Provides superior memory performance characteristics
- Reduces allocation overhead

**AHash (ahash)**
- High-performance hashing algorithm
- 2x faster than standard library HashMap
- Used for internal hash-based data structures

## Compression Support (async-compression)

**Supported algorithms:**
- Brotli compression/decompression
- Gzip compression/decompression
- Zstd compression/decompression
- Integrated with Tokio async runtime

## Serialization & Data Handling

**Serde ecosystem**
- JSON serialization/deserialization (serde_json)
- URL-encoded form data (serde_urlencoded)
- Core serialization traits (serde, serde_core)

**URL Processing**
- URL parsing and manipulation (url crate)
- IDNA support for internationalized domain names
- Percent encoding/decoding

## TLS/SSL Support

**Native TLS**
- OpenSSL bindings (openssl, openssl-sys)
- Rustls for pure-Rust TLS (rustls-pemfile)
- Certificate and key file handling

## Development Tools

**CLI Dependencies (cli/ crate)**
- dialoguer: Interactive prompts and user input
- fs_extra: Enhanced filesystem operations for project scaffolding
- tar, flate2: Archive extraction for templates
- console: Colored terminal output
- tempfile: Temporary file handling

## Build-Time Dependencies

**Procedural Macros**
- syn, quote, proc-macro2: Macro parsing and code generation
- Various derive macros for reducing boilerplate

**Build Scripts**
- cc: C/C++ compilation support
- pkg-config: System library detection
- autocfg: Compiler feature detection

## String/Text Processing

**Unicode Support**
- ICU (International Components for Unicode) for normalization and properties
- UTF-8 iteration utilities
- Unicode width calculations for terminal output