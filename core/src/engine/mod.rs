pub mod parser;

pub use parser::{parse_project_files, RouteSegment, ProjectFile};

pub mod runtime;

pub mod server;
pub use server::Server;

pub mod server_hyper;

pub mod handler;
pub use handler::Response;

pub mod devx;
pub use devx::{RequestContext, ResponseBuilder, responses};

pub mod hot_reload;
pub use hot_reload::HotReloader;

pub mod errors;
pub use errors::{FrameworkError, Result};

pub mod middleware;
pub use middleware::{ResponseCompressor, QueryParser, CorsMiddleware, RateLimiter};

pub mod request;
pub use request::{Request, BodyParser};

pub mod auth;
pub use auth::{Session, SessionStore, JwtAuth, BasicAuth};

pub mod static_files;
pub use static_files::{StaticFileServer, StaticFile};

// Modern WebSocket support with hyper-tungstenite
pub mod ws;
pub use ws::{WebSocketConnection, WebSocket, WsMessage, WsRoom, is_websocket_upgrade, upgrade_websocket, WsSender, WsReceiver};

// User-friendly middleware system
pub mod mw;
pub use mw::{MiddlewareContext, MiddlewareResult, MiddlewareChain, AfterMiddlewareChain};
pub use mw::{logging_middleware, cors_middleware, timing_middleware, auth_middleware};

pub mod config;
pub use config::Config;

pub mod extractors;
pub use extractors::{Json, RawBody, Form, Text};
