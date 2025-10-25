// ✅ REAL EXAMPLE: File Upload with Validation
use std::collections::HashMap;
use serde::Serialize;
use core::engine::BodyParser;

#[derive(Serialize)]
struct UploadResponse {
    success: bool,
    url: String,
    size: usize,
    filename: String,
    content_type: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

pub fn POST(params: &HashMap<String, String>) -> (String, u16) {
    // ✅ REAL FEATURE: Binary Body Parsing
    // Mock binary data (in production, this comes from multipart/form-data)
    let mock_file_data = b"Mock PDF file content...".repeat(100);
    let filename = "document.pdf";
    
    // Parse binary body
    let file_bytes = match BodyParser::bytes(mock_file_data.as_slice()) {
        Ok(bytes) => bytes,
        Err(e) => {
            let error = ErrorResponse { error: format!("Failed to read file: {}", e) };
            return (serde_json::to_string(&error).unwrap(), 400);
        }
    };
    
    let file_size = file_bytes.len();
    
    // ✅ Validation: File size limit (10MB)
    const MAX_SIZE: usize = 10 * 1024 * 1024;
    if file_size > MAX_SIZE {
        let error = ErrorResponse { 
            error: format!("File too large: {} bytes (max: {} bytes)", file_size, MAX_SIZE) 
        };
        return (serde_json::to_string(&error).unwrap(), 413);
    }
    
    // ✅ Validation: File type (basic check by extension)
    let allowed_extensions = ["pdf", "doc", "docx", "txt", "jpg", "png"];
    let extension = filename.split('.').last().unwrap_or("");
    
    if !allowed_extensions.contains(&extension) {
        let error = ErrorResponse {
            error: format!("Invalid file type: .{} (allowed: {})", 
                extension, 
                allowed_extensions.join(", "))
        };
        return (serde_json::to_string(&error).unwrap(), 415);
    }
    
    // Generate unique filename
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    
    let unique_filename = format!("{}_{}.{}", 
        timestamp,
        filename.split('.').next().unwrap_or("file"),
        extension
    );
    
    let file_path = format!("/uploads/2025/10/{}", unique_filename);
    
    // In production: Save to disk or cloud storage
    println!("✅ File upload processed:");
    println!("   Original: {}", filename);
    println!("   Saved as: {}", unique_filename);
    println!("   Size: {} bytes ({:.2} KB)", file_size, file_size as f64 / 1024.0);
    println!("   Type: {}", extension);
    
    let response = UploadResponse {
        success: true,
        url: file_path,
        size: file_size,
        filename: unique_filename,
        content_type: match extension {
            "pdf" => "application/pdf",
            "jpg" | "jpeg" => "image/jpeg",
            "png" => "image/png",
            "txt" => "text/plain",
            _ => "application/octet-stream",
        }.to_string(),
    };
    
    (serde_json::to_string(&response).unwrap(), 201)
}
