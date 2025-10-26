#![allow(dead_code)]

use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt::Write as FmtWrite;

const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const GREEN: &str = "\x1b[32m";
const BLUE: &str = "\x1b[34m";

// Return a compact timestamp with seconds and milliseconds to better trace events.
fn now_ts() -> String {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => {
            let secs = d.as_secs();
            let ms = d.subsec_millis();
            let mut s = String::with_capacity(16);
            let _ = write!(&mut s, "{}.{}", secs, ms);
            s
        }
        Err(_) => "0".to_string(),
    }
}

fn log_to<F: Fn(&str)>(level_col: &str, level_txt: &str, f: F) {
    let ts = now_ts();
    let mut buf = String::with_capacity(128);
    let _ = write!(&mut buf, "{}[{}]{} [{}] ", level_col, level_txt, RESET, ts);
    f(&buf);
}

pub fn info(message: &str) {
    log_to(BLUE, "INFO", |prefix| println!("{}{}", prefix, message));
}

pub fn success(message: &str) {
    log_to(GREEN, "OK", |prefix| println!("{}{}", prefix, message));
}

pub fn warn(message: &str) {
    log_to(YELLOW, "WARN", |prefix| eprintln!("{}{}", prefix, message));
}

pub fn error(message: &str) {
    log_to(RED, "ERR", |prefix| eprintln!("{}{}", prefix, message));
}
