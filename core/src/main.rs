mod engine;
use engine::{parse_project_files, Server, HotReloader};
use colored::Colorize;

// Use jemalloc as the global allocator for better performance
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[tokio::main]
async fn main() {
    // Initialize structured logging from environment (RUST_LOG). Default to info.
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();

    println!("\n{}", "🚀 Framework Starting...".bright_cyan().bold());
    println!("{}", "═".repeat(50).bright_blue());

    let project_files = parse_project_files();
    
    if project_files.is_empty() {
        log::warn!("{} No project files found!", "⚠️".bright_yellow());
    } else {
        println!("\n{} Found {} route file(s):", 
            "📁".bright_green(), 
            project_files.len().to_string().bright_white().bold()
        );
        
        for (idx, project_file) in project_files.iter().enumerate() {
            let route_type = match project_file.file_type.as_str() {
                "api" => "API".bright_blue(),
                "ui" => "UI".bright_magenta(),
                _ => "OTHER".bright_yellow(),
            };
            println!("  {}. [{}] {}", 
                (idx + 1).to_string().bright_white(),
                route_type,
                project_file.file_path.bright_cyan()
            );
        }
    }

    // Start server for manual testing
    let port = 8080;
    let host = "127.0.0.1".to_string();
    let dev_mode = std::env::var("DEV").is_ok() || cfg!(debug_assertions);
    
    println!("\n{}", "═".repeat(50).bright_blue());
    println!("{} Server Configuration:", "⚙️".bright_green());
    println!("  • Address: {}:{}", host.bright_white(), port.to_string().bright_white().bold());
    println!("  • Mode: {}", if dev_mode { 
        "Development".bright_yellow().bold() 
    } else { 
        "Production".bright_green().bold() 
    });
    println!("  • Hot Reload: {}", if dev_mode { 
        "Enabled ♻️".bright_green() 
    } else { 
        "Disabled".bright_red() 
    });
    println!("{}", "═".repeat(50).bright_blue());

    // Enable hot reload in dev mode
    if dev_mode {
        if let Ok(watch_path) = std::env::current_dir() {
            let hot_reloader = HotReloader::new(watch_path.to_string_lossy().to_string());
            if let Err(e) = hot_reloader.start() {
                log::warn!("{} Hot reload failed to start: {}", "⚠️".bright_yellow(), e);
            }
        }
    }

    let server = Server::new(port, host.clone(), dev_mode);
    
    println!("\n{} Server listening on {}http://{}:{}{}", 
        "✓".bright_green().bold(),
        "".bright_white(),
        host.bright_cyan(),
        port.to_string().bright_cyan().bold(),
        "".bright_white()
    );
    println!("{} Press Ctrl+C to stop\n", "ℹ".bright_blue());

    server.start().await;
}
