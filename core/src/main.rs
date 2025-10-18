mod engine;
mod logger;
use engine::{parse_project_files, Server};

#[tokio::main]
async fn main() {
    let project_files = parse_project_files();
    for project_file in project_files {
        logger::info(&format!("Found project file: {}", project_file.file_path));
    }

    // Start server for manual testing
    let server = Server::new(8080, "127.0.0.1".to_string(), true);
    logger::info(&format!("Starting server on {}:{}", server.http_server.host, server.http_server.port));
    server.start().await;
}
