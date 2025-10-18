use std::time::{SystemTime, UNIX_EPOCH};

const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const GREEN: &str = "\x1b[32m";
const BLUE: &str = "\x1b[34m";

fn now_ts() -> String {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => format!("{}", d.as_secs()),
        Err(_) => "0".to_string(),
    }
}

pub fn info(message: &str) {
    println!("{}[INFO]{} [{}] {}", BLUE, RESET, now_ts(), message);
}

pub fn success(message: &str) {
    println!("{}[OK]{}   [{}] {}", GREEN, RESET, now_ts(), message);
}

pub fn warn(message: &str) {
    eprintln!("{}[WARN]{} [{}] {}", YELLOW, RESET, now_ts(), message);
}

pub fn error(message: &str) {
    eprintln!("{}[ERR]{}  [{}] {}", RED, RESET, now_ts(), message);
}
