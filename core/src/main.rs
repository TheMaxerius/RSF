mod engine;
use engine::{parse_project_files, Server, HotReloader, Config};
use colored::Colorize;

// Use jemalloc as the global allocator for better performance
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[tokio::main]
async fn main() {
    // Load configuration from environment
    let config = Config::from_env();
    
    // Initialize structured logging from environment (RUST_LOG)
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(&config.log_level))
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

    // Display server configuration
    println!("\n{}", "═".repeat(50).bright_blue());
    println!("{} Server Configuration:", "⚙️".bright_green());
    println!("  • Address: {}:{}", config.host.bright_white(), config.port.to_string().bright_white().bold());
    println!("  • Mode: {}", if config.dev_mode { 
        "Development".bright_yellow().bold() 
    } else { 
        "Production".bright_green().bold() 
    });
    println!("  • Hot Reload: {}", if config.hot_reload { 
        "Enabled ♻️".bright_green() 
    } else { 
        "Disabled".bright_red() 
    });
    println!("  • Log Level: {}", config.log_level.bright_white());
    println!("{}", "═".repeat(50).bright_blue());

    // Enable hot reload if configured
    if config.hot_reload {
        if let Ok(watch_path) = std::env::current_dir() {
            let hot_reloader = HotReloader::new(watch_path.to_string_lossy().to_string());
            if let Err(e) = hot_reloader.start() {
                log::warn!("{} Hot reload failed to start: {}", "⚠️".bright_yellow(), e);
            }
        }
    }

    let server = Server::new(config.port, config.host.clone(), config.dev_mode);
    
    println!("\n{} Server listening on {}http://{}:{}{}", 
        "✓".bright_green().bold(),
        "".bright_white(),
        config.host.bright_cyan(),
        config.port.to_string().bright_cyan().bold(),
        "".bright_white()
    );
    println!("{} Press Ctrl+C to stop\n", "ℹ".bright_blue());

    server.start().await;
}
