use std::path::PathBuf;
use dialoguer::{Input, Select};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let choices = vec!["new", "dev", "edit", "quit"];
    let selection = Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt("Choose command")
        .items(&choices)
        .default(0)
        .interact()?;

    match choices[selection] {
        "new" => cmd_new().await?,
        "dev" => cmd_dev().await?,
        "edit" => cmd_edit()?,
        _ => println!("bye"),
    }

    Ok(())
}

async fn cmd_new() -> Result<()> {
    let project_name: String = Input::new().with_prompt("Project name").interact_text()?;
    let project_root = PathBuf::from(format!("./{}", project_name));
    
    if project_root.exists() {
        println!("Destination {} already exists", project_name);
        return Ok(());
    }

    println!("Downloading template...");
    
    // Download the template tarball from GitHub
    let template_url = "https://github.com/TheMaxerius/framework_test/archive/refs/heads/master.tar.gz";
    let response = reqwest::Client::new()
        .get(template_url)
        .send()
        .await?;
    
    if !response.status().is_success() {
        anyhow::bail!("Failed to download template: {}", response.status());
    }
    
    let bytes = response.bytes().await?;
    
    // Extract to temp directory
    let temp_dir = std::env::temp_dir().join(format!("framework_template_{}", std::process::id()));
    if temp_dir.exists() {
        std::fs::remove_dir_all(&temp_dir)?;
    }
    std::fs::create_dir_all(&temp_dir)?;
    
    // Decompress and extract tar.gz
    let cursor = std::io::Cursor::new(bytes);
    let gz = flate2::read::GzDecoder::new(cursor);
    let mut archive = tar::Archive::new(gz);
    archive.unpack(&temp_dir)?;
    
    // Find the extracted folder (could be framework_test-master or framework-template-main, etc)
    let mut entries: Vec<_> = std::fs::read_dir(&temp_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .collect();
    
    if entries.is_empty() {
        println!("Temp dir contents:");
        for entry in std::fs::read_dir(&temp_dir)? {
            if let Ok(e) = entry {
                println!("  {:?}", e.path());
            }
        }
        anyhow::bail!("No directories found in extracted template");
    }
    
    let extracted_dir = entries.remove(0).path();
    println!("Using template from: {:?}", extracted_dir);
    
    // Copy extracted template to project root
    std::fs::create_dir_all(&project_root)?;
    copy_dir_all(&extracted_dir, &project_root)?;
    
    println!("Created project {} at {:?}", project_name, project_root);
    
    // Create user src directory
    let user_src_dir = project_root.join("src");
    std::fs::create_dir_all(&user_src_dir)?;
    
    // Create example main.rs
    let main_rs_content = r#"//'api'
use std::collections::HashMap;

// Handler for GET /main
pub fn GET(_params: &HashMap<String, String>) -> String {
    "Hello from example main".to_string()
}"#;
    std::fs::write(user_src_dir.join("main.rs"), main_rs_content)?;
    println!("Created user src directory");
    
    // Update project.json with correct path
    let pj_path = project_root.join("core/src/engine/project.json");
    if pj_path.exists() {
        let mut pj: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&pj_path)?)?;
        pj["parent_folder"] = serde_json::Value::String(format!("../../../{}", project_name));
        std::fs::write(&pj_path, serde_json::to_string_pretty(&pj)?)?;
    }
    
    // Cleanup
    std::fs::remove_dir_all(&temp_dir)?;
    
    println!("\n✓ Project created successfully!");
    println!("\nNext steps:");
    println!("1. cd {}", project_name);
    println!("2. Run 'cli' and select 'edit' to configure the project");
    println!("3. Run 'cli' and select 'dev' to start the development server");
    
    Ok(())
}

fn copy_dir_all(src: &PathBuf, dst: &PathBuf) -> Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let file_name = entry.file_name();
        let dst_path = dst.join(&file_name);

        if ty.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

async fn cmd_dev() -> Result<()> {
    println!("Starting dev server in core/");
    let mut cmd = tokio::process::Command::new("cargo");
    cmd.arg("run").current_dir("core");
    let status = cmd.status().await?;
    println!("cargo run exited with {}", status);
    Ok(())
}

fn cmd_edit() -> Result<()> {
    let parent: String = Input::new()
        .with_prompt("New parent_folder for project.json (relative to core/src/engine)")
        .interact_text()?;

    let pj = PathBuf::from("core/src/engine/project.json");
    let mut cfg: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&pj)?)?;
    cfg["parent_folder"] = serde_json::Value::String(format!("../../../{}", parent.clone()));
    std::fs::write(&pj, serde_json::to_string_pretty(&cfg)?)?;

    println!("Updated project.json parent_folder to {}", parent);
    Ok(())
}