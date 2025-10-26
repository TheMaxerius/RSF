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

        // Emit module blocks by inlining the discovered .rs file contents. For each file we create a
        // private `__orig` submodule that contains the raw file contents, then emit public wrapper
        // shims inside the parent module that adapt various handler signatures to the project's
        // expected `Handler = fn(&HashMap<String,String>) -> String` type.
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

            writeln!(out, "#[allow(non_snake_case)]").unwrap();
            writeln!(out, "mod {} {{", mod_name).unwrap();

            // Inline original file inside a private `__orig` module to avoid name collisions.
            writeln!(out, "    mod __orig {{").unwrap();
            for line in content.lines() {
                // Promote private top-level function declarations to pub(crate) so parent
                // wrappers can call them. Handle `fn`, `async fn`, and skip already-pub lines.
                let trimmed = line.trim_start();
                if trimmed.starts_with("pub ") || trimmed.starts_with("#") || trimmed.starts_with("use ") {
                    writeln!(out, "        {}", line).unwrap();
                } else if trimmed.starts_with("async fn ") {
                    let indent = &line[..line.len() - trimmed.len()];
                    // emit `pub(crate) async fn <rest>`
                    writeln!(out, "        {}pub(crate) async fn {}", indent, &trimmed[9..]).unwrap();
                } else if trimmed.starts_with("fn ") {
                    let indent = &line[..line.len() - trimmed.len()];
                    // emit `pub(crate) fn <rest>`
                    writeln!(out, "        {}pub(crate) fn {}", indent, &trimmed[3..]).unwrap();
                } else {
                    writeln!(out, "        {}", line).unwrap();
                }
            }
            writeln!(out, "    }}").unwrap();

            // Detect handler functions and emit public wrapper shims that adapt signatures.
            let methods = ["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS", "HEAD"];
            for m in &methods {
                let upper_pat = format!("fn {}(", m);
                let lower_pat = format!("fn {}(", m.to_lowercase());
                if content.contains(&upper_pat) || content.contains(&lower_pat) {
                    // crude detection whether the original returns (String,u16)
                    let returns_tuple = content.contains(&format!("fn {}(", m)) && content.contains("-> (String,");
                    // detect whether the original function takes a reference parameter
                    let param_is_ref = content.contains(": &HashMap") || content.contains(":&HashMap");

                    // Detect if the original handler accepts body parameter
                    // Check both uppercase and lowercase function names (e.g., fn POST or fn post)
                    let fn_sig_uppercase = format!("fn {}(", m);
                    let fn_sig_lowercase = format!("fn {}(", m.to_lowercase());
                    
                    let accepts_body = {
                        let mut found = false;
                        for pattern in &[fn_sig_uppercase, fn_sig_lowercase] {
                            if let Some(fn_pos) = content.find(pattern) {
                                if let Some(closing_paren) = content[fn_pos..].find(')') {
                                    let signature = &content[fn_pos..fn_pos + closing_paren];
                                    if signature.contains("body:") || signature.contains("_body:") || signature.contains(", body") {
                                        found = true;
                                        break;
                                    }
                                }
                            }
                        }
                        found
                    };
                    
                    // Also detect if the original already returns a Response
                    let returns_response = content.contains("-> Response") || content.contains("-> super::Response") || content.contains("-> crate::engine::Response");
                    if returns_response {
                        writeln!(out, "    // async wrapper for {} that forwards Response", m).unwrap();
                        writeln!(out, "    #[inline(always)]").unwrap();
                        writeln!(out, "    pub fn {}(params: &std::collections::HashMap<String, String>, body: &bytes::Bytes) -> std::pin::Pin<Box<dyn std::future::Future<Output = super::Response> + Send>> {{", m).unwrap();
                        writeln!(out, "        let params = params.clone();").unwrap();
                        writeln!(out, "        let body = body.clone();").unwrap();
                        writeln!(out, "        Box::pin(async move {{").unwrap();
                        if content.contains(&upper_pat) {
                            if accepts_body {
                                writeln!(out, "            __orig::{}(&params, &body).await", m).unwrap();
                            } else {
                                writeln!(out, "            __orig::{}(&params).await", m).unwrap();
                            }
                        } else {
                            let lname = m.to_lowercase();
                            if accepts_body {
                                writeln!(out, "            __orig::{}(&params, &body).await", lname).unwrap();
                            } else {
                                writeln!(out, "            __orig::{}(&params).await", lname).unwrap();
                            }
                        }
                        writeln!(out, "        }})").unwrap();
                        writeln!(out, "    }}").unwrap();
                    } else if returns_tuple {
                        writeln!(out, "    // async wrapper for {} that adapts (String,u16) -> Response", m).unwrap();
                        writeln!(out, "    #[inline(always)]").unwrap();
                        writeln!(out, "    pub fn {}(params: &std::collections::HashMap<String, String>, body: &bytes::Bytes) -> std::pin::Pin<Box<dyn std::future::Future<Output = super::Response> + Send>> {{", m).unwrap();
                        writeln!(out, "        let params = params.clone();").unwrap();
                        writeln!(out, "        let body = body.clone();").unwrap();
                        writeln!(out, "        Box::pin(async move {{").unwrap();
                        if content.contains(&upper_pat) {
                            if accepts_body {
                                writeln!(out, "            let (s, status) = __orig::{}(&params, &body).await;", m).unwrap();
                            } else {
                                writeln!(out, "            let (s, status) = __orig::{}(&params).await;", m).unwrap();
                            }
                        } else {
                            let lname = m.to_lowercase();
                            if accepts_body {
                                writeln!(out, "            let (s, status) = __orig::{}(&params, &body).await;", lname).unwrap();
                            } else {
                                writeln!(out, "            let (s, status) = __orig::{}(&params).await;", lname).unwrap();
                            }
                        }
                        writeln!(out, "            super::Response {{ status, body: s.into_bytes().into(), content_type: \"text/plain; charset=utf-8\", headers: Vec::new() }}").unwrap();
                        writeln!(out, "        }})").unwrap();
                        writeln!(out, "    }}").unwrap();
                    } else {
                        writeln!(out, "    // async wrapper for {} assuming it returns String", m).unwrap();
                        writeln!(out, "    #[inline(always)]").unwrap();
                        writeln!(out, "    pub fn {}(params: &std::collections::HashMap<String, String>, body: &bytes::Bytes) -> std::pin::Pin<Box<dyn std::future::Future<Output = super::Response> + Send>> {{", m).unwrap();
                        writeln!(out, "        let params = params.clone();").unwrap();
                        writeln!(out, "        let body = body.clone();").unwrap();
                        writeln!(out, "        Box::pin(async move {{").unwrap();
                        if content.contains(&upper_pat) {
                            if accepts_body {
                                writeln!(out, "            let s = __orig::{}(&params, &body).await;", m).unwrap();
                            } else {
                                writeln!(out, "            let s = __orig::{}(&params).await;", m).unwrap();
                            }
                        } else {
                            let lname = m.to_lowercase();
                            if accepts_body {
                                writeln!(out, "            let s = __orig::{}(&params, &body).await;", lname).unwrap();
                            } else {
                                writeln!(out, "            let s = __orig::{}(&params).await;", lname).unwrap();
                            }
                        }
                        writeln!(out, "            super::Response {{ status: 200, body: s.into_bytes().into(), content_type: \"text/plain; charset=utf-8\", headers: Vec::new() }}").unwrap();
                        writeln!(out, "        }})").unwrap();
                        writeln!(out, "    }}").unwrap();
                    }
                }
            }

            writeln!(out, "}}\n").unwrap();
        }

    // Emit get_handler function or a minimal stub if no files were found
    if files.is_empty() {
        // overwrite out with a minimal stub
        let mut out = fs::File::create(&out_path).expect("Failed to create generated_routes.rs");
        writeln!(out, "// GENERATED FILE - DO NOT EDIT\n").unwrap();
        writeln!(out, "use std::collections::HashMap;\n").unwrap();
        writeln!(out, "use std::pin::Pin;").unwrap();
        writeln!(out, "use std::future::Future;").unwrap();
        writeln!(out, "pub type Handler = fn(&HashMap<String, String>, &bytes::Bytes) -> Pin<Box<dyn Future<Output = super::Response> + Send>>;\n").unwrap();
        writeln!(out, "pub fn get_handler(_route: &str, _method: &str) -> Option<(Handler, HashMap<String, String>)> {{ None }}\n").unwrap();
    } else {
    writeln!(out, "use std::option::Option;").unwrap();
        writeln!(out, "use std::pin::Pin;").unwrap();
        writeln!(out, "use std::future::Future;\n").unwrap();
        writeln!(out, "pub type Handler = fn(&std::collections::HashMap<String, String>, &bytes::Bytes) -> Pin<Box<dyn Future<Output = super::Response> + Send>>;\n").unwrap();
        
        // Generate a route matcher that handles both static and dynamic routes
        writeln!(out, "#[inline(always)]").unwrap();
        writeln!(out, "pub fn get_handler(route: &str, method: &str) -> Option<(Handler, std::collections::HashMap<String, String>)> {{").unwrap();
        writeln!(out, "    // Fast path: pre-check method bytes for quick rejection").unwrap();
        writeln!(out, "    let method_bytes = method.as_bytes();").unwrap();
        writeln!(out, "    ").unwrap();
        writeln!(out, "    // Normalize and split route into segments (stack-allocated for small routes)").unwrap();
        writeln!(out, "    let route_normalized = route.trim_start_matches('/').trim_end_matches('/');").unwrap();
        writeln!(out, "    let seg_count = if route_normalized.is_empty() {{ 0 }} else {{ route_normalized.bytes().filter(|&b| b == b'/').count() + 1 }};").unwrap();
        writeln!(out, "    ").unwrap();
        writeln!(out, "    // Use small fixed arrays for common cases to avoid heap allocation").unwrap();
        writeln!(out, "    let mut seg_buf: [&str; 8] = [\"\"; 8];").unwrap();
        writeln!(out, "    let segments = if seg_count <= 8 {{").unwrap();
        writeln!(out, "        let mut i = 0;").unwrap();
        writeln!(out, "        for seg in route_normalized.split('/') {{").unwrap();
        writeln!(out, "            if i >= 8 {{ break; }}").unwrap();
        writeln!(out, "            seg_buf[i] = seg;").unwrap();
        writeln!(out, "            i += 1;").unwrap();
        writeln!(out, "        }}").unwrap();
        writeln!(out, "        &seg_buf[..seg_count]").unwrap();
        writeln!(out, "    }} else {{").unwrap();
        writeln!(out, "        // Fallback to heap for deep routes").unwrap();
        writeln!(out, "        return None; // or handle with Vec if needed").unwrap();
        writeln!(out, "    }};").unwrap();
        writeln!(out).unwrap();

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
                let has_handler = content.contains(&upper_pat) || content.contains(&lower_pat);
                
                if has_handler {
                    let fname = if content.contains(&upper_pat) { m.to_string() } else { m.to_lowercase() };
                    
                    // Parse route pattern
                    let route_pattern = route.trim_start_matches('/').trim_end_matches('/');
                    let pattern_segments: Vec<&str> = if route_pattern.is_empty() { 
                        Vec::new() 
                    } else { 
                        route_pattern.split('/').collect() 
                    };
                    
                    writeln!(out, "    // Match pattern: {} {}", m, route).unwrap();
                    
                    // Optimize method check with byte comparison
                    let method_bytes: Vec<String> = m.bytes().map(|b| format!("{}", b)).collect();
                    let method_check = if method_bytes.len() <= 7 {
                        format!("method_bytes.len() == {} && method_bytes == b\"{}\"", m.len(), m)
                    } else {
                        format!("method == \"{}\"", m)
                    };
                    
                    writeln!(out, "    if {} && seg_count == {} {{", method_check, pattern_segments.len()).unwrap();
                    
                    // Generate matching logic with optimized segment access
                    let mut has_dynamics = false;
                    let mut static_checks = Vec::new();
                    
                    for (i, seg) in pattern_segments.iter().enumerate() {
                        if seg.starts_with('[') && seg.ends_with(']') {
                            has_dynamics = true;
                        } else {
                            static_checks.push((i, seg));
                        }
                    }
                    
                    // Generate static segment checks - use direct array access for speed
                    for (i, seg) in &static_checks {
                        writeln!(out, "        if segments[{}] != \"{}\" {{ /* skip */ }} else", i, seg).unwrap();
                    }
                    
                    writeln!(out, "        {{").unwrap();
                    
                    // Extract dynamic params with pre-allocated capacity
                    if has_dynamics {
                        let param_count = pattern_segments.iter().filter(|s| s.starts_with('[') && s.ends_with(']')).count();
                        writeln!(out, "            let mut params = std::collections::HashMap::with_capacity({});", param_count).unwrap();
                        for (i, seg) in pattern_segments.iter().enumerate() {
                            if seg.starts_with('[') && seg.ends_with(']') {
                                let param_name = &seg[1..seg.len()-1];
                                // Use unchecked access since we know seg_count == pattern_segments.len()
                                writeln!(out, "            params.insert(\"{}\".to_string(), segments[{}].to_string());", param_name, i).unwrap();
                            }
                        }
                        writeln!(out, "            return Some(({}::{}, params));", mod_name, fname).unwrap();
                    } else {
                        writeln!(out, "            return Some(({}::{}, std::collections::HashMap::new()));", mod_name, fname).unwrap();
                    }
                    
                    writeln!(out, "        }}").unwrap();
                    writeln!(out, "    }}").unwrap();
                }
            }
        }

        writeln!(out, "    None").unwrap();
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
    // Also handle dynamic segments like [id] -> _id_
    let s = path.to_string_lossy();
    let s = s.replace("/", "_").replace("\\\\", "_");
    let s = s.replace('.', "_");
    let s = s.replace('[', "_").replace(']', "_");
    let s = s.replace('-', "_");
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
