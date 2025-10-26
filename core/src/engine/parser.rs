use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

use serde::Deserialize;

/// Represents a file found in the project and whether it's a UI or API file.
#[derive(Debug, Clone)]
pub struct ProjectFile {
    /// The path relative to the configured parent folder (e.g. "index.rs" or "admin/users.rs").
    pub file_path: String,
    /// The absolute filesystem path to the file; used when serving the file contents.
    pub full_path: String,
    pub file_type: String, // "ui" or "api"
    /// Precomputed route segments for fast matching (avoid splitting on every request)
    pub route_segments: Vec<RouteSegment>,
}

/// A route segment that can be either static or dynamic (param)
#[derive(Debug, Clone)]
pub enum RouteSegment {
    Static(String),
    Dynamic(String), // param name
}

#[derive(Deserialize)]
struct ProjectConfig {
    parent_folder: String,
}

fn read_project_configurations(file_path: &str) -> Option<ProjectConfig> {
    // Try to read the file at runtime first (current working directory)
    if let Ok(content) = fs::read_to_string(file_path) {
        if let Ok(cfg) = serde_json::from_str::<ProjectConfig>(&content) {
            return Some(cfg);
        }
    }

    // Try to read project.json relative to the crate (useful when running from other CWDs)
    let crate_project = Path::new(env!("CARGO_MANIFEST_DIR")).join("src").join("engine").join(file_path);
    if let Ok(content) = fs::read_to_string(&crate_project) {
        if let Ok(cfg) = serde_json::from_str::<ProjectConfig>(&content) {
            return Some(cfg);
        }
    }

    // Fall back to the embedded project.json at compile time
    serde_json::from_str(include_str!("project.json")).ok()
}

fn read_first_line(file_path: &str) -> String {
    if let Ok(file) = fs::File::open(file_path) {
        let reader = io::BufReader::new(file);
        if let Some(Ok(line)) = reader.lines().next() {
            return line;
        }
    }
    String::new()
}

/// Compute route segments from a file path for fast matching.
/// Handles index.rs and dynamic segments like [id].
fn compute_route_segments(file_path: &str) -> Vec<RouteSegment> {
    // Normalize file path to route segments
    let fp = file_path.trim();
    // handle index.rs specially
    let route_fp = if fp.ends_with("index.rs") {
        let dir = fp.trim_end_matches("index.rs").trim_end_matches('/');
        dir.trim_end_matches('/')
    } else {
        fp.trim_end_matches(".rs")
    };

    if route_fp.is_empty() {
        return Vec::new();
    }

    route_fp.split('/')
        .map(|seg| {
            if seg.starts_with('[') && seg.ends_with(']') {
                let name = &seg[1..seg.len() - 1];
                RouteSegment::Dynamic(name.to_string())
            } else {
                RouteSegment::Static(seg.to_string())
            }
        })
        .collect()
}

// Older non-recursive helpers removed; parse_rs_files_in_folder_recursive is used instead.

fn resolve_parent_folder(parent_folder: &str) -> String {
    let parent_path = Path::new(parent_folder);
    if parent_path.is_absolute() {
        return parent_folder.to_string();
    }

    // First try relative to current working directory
    if let Ok(cwd) = std::env::current_dir() {
        let cand = cwd.join(parent_path);
        if cand.exists() {
            return match fs::canonicalize(&cand) {
                Ok(p) => p.to_str().map(|s| s.to_string()).unwrap_or_else(|| cand.to_string_lossy().into_owned()),
                Err(_) => cand.to_string_lossy().into_owned(),
            };
        }
    }

    // Next try relative to the crate's src/engine (useful when running from other CWDs)
    let project_json_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src").join("engine");
    let joined = project_json_dir.join(parent_path);
    if joined.exists() {
        return match fs::canonicalize(&joined) {
            Ok(p) => p.to_str().map(|s| s.to_string()).unwrap_or_else(|| joined.to_string_lossy().into_owned()),
            Err(_) => joined.to_string_lossy().into_owned(),
        };
    }

    // Fallback: return the parent_folder as-is
    parent_folder.to_string()
}

/// Parse project.json and return list of ProjectFile from the configured parent folder.
pub fn parse_project_files() -> Vec<ProjectFile> {
    let config = read_project_configurations("project.json");
    let parent_folder = config
        .as_ref()
        .map(|c| c.parent_folder.as_str())
        .unwrap_or(".");

    let resolved = resolve_parent_folder(parent_folder);
    parse_rs_files_in_folder_recursive(&resolved)
}

fn parse_rs_files_in_folder_recursive(folder_path: &str) -> Vec<ProjectFile> {
    let mut project_files = Vec::new();
    let start = Path::new(folder_path);
    if !start.exists() {
        return project_files;
    }

    let mut stack = vec![start.to_path_buf()];
    while let Some(dir) = stack.pop() {
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                    continue;
                }

                if let Some(ext) = path.extension() {
                    if ext == "rs" {
                        // read first line from the full path
                        let path_str = path.to_string_lossy().to_string();
                        let first_line = read_first_line(&path_str);
                        let file_type = if first_line.contains("ui") {
                            "ui"
                        } else if first_line.contains("api") {
                            "api"
                        } else {

                            "other"

                        };

                        // compute path relative to the configured parent folder
                        let rel = match path.strip_prefix(&start) {
                            Ok(p) => p.to_path_buf(),
                            Err(_) => path.clone(),
                        };
                        // normalize to forward slashes and store the relative path (with directories)
                        if let Some(rel_str) = rel.to_str() {
                            let normalized = rel_str.replace("\\", "/");
                            if file_type == "ui" || file_type == "api" {
                                // compute absolute full path for serving later
                                let full = match fs::canonicalize(&path) {
                                    Ok(p) => p.to_string_lossy().to_string(),
                                    Err(_) => path.to_string_lossy().to_string(),
                                };
                                // Precompute route segments for fast matching
                                let route_segments = compute_route_segments(&normalized);
                                project_files.push(ProjectFile { 
                                    file_path: normalized, 
                                    full_path: full, 
                                    file_type: file_type.into(),
                                    route_segments,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    project_files
}

