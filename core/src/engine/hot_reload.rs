// Hot reload functionality for development
use notify::{Watcher, RecursiveMode, Result as NotifyResult, Event};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;
use colored::Colorize;

pub struct HotReloader {
    watch_path: String,
}

impl HotReloader {
    pub fn new(watch_path: String) -> Self {
        Self { watch_path }
    }

    pub fn start(&self) -> NotifyResult<()> {
        let (tx, rx) = channel();
        
        let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res {
                let _ = tx.send(event);
            }
        })?;

        watcher.watch(Path::new(&self.watch_path), RecursiveMode::Recursive)?;
        
        log::info!("{} Watching {} for changes...", 
            "🔥".bright_yellow(), 
            self.watch_path.bright_cyan()
        );

        std::thread::spawn(move || {
            loop {
                match rx.recv_timeout(Duration::from_millis(100)) {
                    Ok(event) => {
                        if let notify::EventKind::Modify(_) | notify::EventKind::Create(_) = event.kind {
                            // Filter to only .rs files, excluding generated and build artifacts
                            let should_reload = event.paths.iter().any(|p| {
                                if !p.extension().map(|e| e == "rs").unwrap_or(false) {
                                    return false;
                                }
                                let path_str = p.to_string_lossy();
                                // Ignore target/, generated files, and build artifacts
                                !path_str.contains("/target/") && 
                                !path_str.contains("generated_routes.rs") &&
                                !path_str.contains("\\target\\")
                            });
                            
                            if should_reload {
                                log::info!("{} File changed: {:?}", 
                                    "♻️".bright_green(),
                                    event.paths.first().map(|p| p.display())
                                );
                                log::warn!("{} Hot reload triggered - restart server to apply changes", 
                                    "⚠️".bright_yellow()
                                );
                            }
                        }
                    }
                    Err(_) => continue,
                }
            }
        });

        // Keep watcher alive
        std::mem::forget(watcher);
        Ok(())
    }
}
