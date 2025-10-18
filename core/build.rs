use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    // Read project.json from crate
    let proj_json = Path::new(env!("CARGO_MANIFEST_DIR")).join("src").join("engine").join("project.json");
    let content = fs::read_to_string(&proj_json).expect("Failed to read project.json");
    let cfg: serde_json::Value = serde_json::from_str(&content).expect("Invalid project.json");
    let parent = cfg.get("parent_folder").and_then(|v| v.as_str()).unwrap_or(".");

    // Resolve absolute parent folder path relative to crate
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src").join("engine");
    let parent_path = crate_dir.join(parent);
    let parent_path = if parent_path.exists() { parent_path } else { Path::new(parent).to_path_buf() };

    // Collect .rs files recursively
    let mut files = Vec::new();
    collect_rs_files(&parent_path, &mut files, &parent_path);

    // Prepare output
    let out_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src").join("engine").join("generated_routes.rs");
    let mut out = fs::File::create(&out_path).expect("Failed to create generated_routes.rs");

    writeln!(out, "// GENERATED FILE - DO NOT EDIT\n").unwrap();

        // Emit module blocks by inlining the discovered .rs file contents. This avoids include! path resolution
        for file in &files {
            let mod_name = module_name_for(file);
            // Read the original file contents
            let content = match fs::read_to_string(&file) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("[build.rs WARN] failed to read {}: {}", file.display(), e);
                    String::new()
                }
            };
            // Emit the module with the file's contents inside
            writeln!(out, "#[allow(non_snake_case)]").unwrap();
            writeln!(out, "mod {} {{", mod_name).unwrap();
            // Write the file contents directly. Ensure it is on new lines.
            for line in content.lines() {
                writeln!(out, "    {}", line).unwrap();
            }
            writeln!(out, "}}\n").unwrap();
        }

    // Emit get_handler function or a minimal stub if no files were found
    if files.is_empty() {
        // overwrite out with a minimal stub
        let mut out = fs::File::create(&out_path).expect("Failed to create generated_routes.rs");
        writeln!(out, "// GENERATED FILE - DO NOT EDIT\n").unwrap();
        writeln!(out, "use std::collections::HashMap;\n").unwrap();
        writeln!(out, "pub type Handler = fn(&HashMap<String, String>) -> String;\n").unwrap();
        writeln!(out, "pub fn get_handler(_route: &str, _method: &str) -> Option<Handler> {{ None }}\n").unwrap();
    } else {
        writeln!(out, "use std::option::Option;\nuse std::boxed::Box;\n").unwrap();
        writeln!(out, "pub type Handler = fn(&std::collections::HashMap<String, String>) -> String;\n").unwrap();
        writeln!(out, "pub fn get_handler(route: &str, method: &str) -> Option<Handler> {{").unwrap();
        writeln!(out, "    match (route, method) {{").unwrap();

        // For each discovered file, detect which HTTP method handler functions it provides
        let methods = ["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS", "HEAD"];
        for file in &files {
            let mod_name = module_name_for(file);
            let route = path_to_route(file.strip_prefix(&parent_path).unwrap().to_str().unwrap());

            // Read file content to scan for handler functions
            let content = match fs::read_to_string(&file) {
                Ok(s) => s,
                Err(_) => String::new(),
            };

            for m in &methods {
                // check both UPPERCASE and lowercase function names
                let upper_pat = format!("fn {}(", m);
                let lower_pat = format!("fn {}(", m.to_lowercase());
                if content.contains(&upper_pat) {
                    writeln!(out, "        (\"{}\", \"{}\") => Some({}::{}),", route, m, mod_name, m).unwrap();
                } else if content.contains(&lower_pat) {
                    let fname = m.to_lowercase();
                    writeln!(out, "        (\"{}\", \"{}\") => Some({}::{}),", route, m, mod_name, fname).unwrap();
                }
            }
        }

        writeln!(out, "        _ => None,").unwrap();
        writeln!(out, "    }}").unwrap();
        writeln!(out, "}}").unwrap();
    }
}

fn collect_rs_files(dir: &Path, files: &mut Vec<std::path::PathBuf>, root: &Path) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                collect_rs_files(&p, files, root);
            } else if let Some(ext) = p.extension() {
                if ext == "rs" {
                    files.push(p);
                }
            }
        }
    }
}

fn module_name_for(path: &Path) -> String {
    // Convert path/to/foo/bar.rs -> path_to_foo_bar
    let s = path.to_string_lossy();
    let s = s.replace("/", "_").replace("\\\\", "_");
    let s = s.replace('.', "_");
    format!("module_{}", s)
}

fn path_to_route(path: &str) -> String {
    // Convert "auth/main.rs" -> "/auth/main"; index.rs -> "/auth" or "/"
    let p = path.replace("\\\\", "/");
    let p = if p.ends_with("index.rs") {
        let dir = p.trim_end_matches("index.rs");
        let dir = dir.trim_end_matches('/');
        if dir.is_empty() { "/".to_string() } else { format!("/{}", dir.trim_start_matches('/')) }
    } else {
        let without = p.trim_end_matches(".rs");
        format!("/{}", without.trim_start_matches('/'))
    };
    p
}
