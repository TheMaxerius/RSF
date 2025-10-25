// ✅ REAL EXAMPLE: Response Compression
use std::collections::HashMap;
use serde::Serialize;
use core::engine::ResponseCompressor;

#[derive(Serialize)]
struct DataPoint {
    id: usize,
    value: f64,
    label: String,
    metadata: String,
}

pub fn GET(params: &HashMap<String, String>) -> (String, u16) {
    // Generate large dataset (perfect for compression)
    let mut data_points = Vec::with_capacity(1000);
    for i in 0..1000 {
        data_points.push(DataPoint {
            id: i,
            value: (i as f64) * 1.5 + 10.0,
            label: format!("Data point #{}", i),
            metadata: format!("Additional metadata for point {} with some extra text to increase size", i),
        });
    }
    
    let json = serde_json::to_string(&data_points).unwrap();
    let original_size = json.len();
    
    // ✅ REAL FEATURE: Gzip Compression
    // Only compress if > 1KB and beneficial
    if let Some(compressed) = ResponseCompressor::compress_gzip(json.as_bytes(), 1024) {
        let compressed_size = compressed.len();
        let ratio = (100.0 * compressed_size as f64) / original_size as f64;
        
        println!("✅ Response compressed!");
        println!("   Original: {} bytes", original_size);
        println!("   Compressed: {} bytes", compressed_size);
        println!("   Compression ratio: {:.1}%", ratio);
        println!("   Savings: {} bytes ({:.1}%)", 
            original_size - compressed_size,
            100.0 - ratio);
        
        // In production, would return compressed with header:
        // Content-Encoding: gzip
        // For demo, return original with stats
        let stats = serde_json::json!({
            "compression": {
                "enabled": true,
                "original_size": original_size,
                "compressed_size": compressed_size,
                "ratio": format!("{:.1}%", ratio),
                "savings_bytes": original_size - compressed_size
            },
            "data_points": 1000,
            "note": "In production, this would be sent as gzip with Content-Encoding header"
        });
        
        return (stats.to_string(), 200);
    }
    
    // Response too small for compression
    println!("✅ Compression skipped (response < 1KB or not beneficial)");
    println!("   Size: {} bytes", original_size);
    
    (json, 200)
}
