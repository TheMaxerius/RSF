use crate::engine::parser::{ProjectFile, parse_project_files};

// Create a runtime for parsed files
#[derive(Clone, Debug)]
pub struct Runtime {
    pub project_files: Vec<ProjectFile>,
    pub port: u16,
    pub host: String,
    pub dev: bool,
    /// Cache mapping full_path -> file contents. Populated at startup in non-dev mode.
    pub file_cache: std::collections::HashMap<String, bytes::Bytes>,
}

impl Runtime {
    pub fn new(port: u16, host: String, dev: bool) -> Self {
        let project_files = parse_project_files();
        let mut cache = std::collections::HashMap::new();
        if !dev {
            for pf in &project_files {
                if let Ok(s) = std::fs::read_to_string(&pf.full_path) {
                    cache.insert(pf.full_path.clone(), bytes::Bytes::from(s));
                }
            }
        }
        Runtime { project_files, port, host, dev, file_cache: cache }
    }
}

