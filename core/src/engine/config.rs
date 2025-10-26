// Configuration management with environment variable support
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub host: String,
    pub dev_mode: bool,
    pub hot_reload: bool,
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> Self {
        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(5000);
        
        let host = env::var("HOST")
            .unwrap_or_else(|_| "0.0.0.0".to_string());
        
        let dev_mode = env::var("DEV").is_ok() || cfg!(debug_assertions);
        
        let hot_reload = env::var("HOT_RELOAD")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(dev_mode);
        
        let log_level = env::var("RUST_LOG")
            .unwrap_or_else(|_| if dev_mode { "debug".to_string() } else { "info".to_string() });
        
        Self {
            port,
            host,
            dev_mode,
            hot_reload,
            log_level,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 5000,
            host: "0.0.0.0".to_string(),
            dev_mode: cfg!(debug_assertions),
            hot_reload: cfg!(debug_assertions),
            log_level: "info".to_string(),
        }
    }
}
