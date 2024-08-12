use std::fs;
use std::path::Path;
use serde_json::Value;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <directory>", args[0]);
        std::process::exit(1);
    }

    let dir_path = &args[1];
    validate_files(dir_path)?;
    Ok(())
}

fn validate_files(dir_path: &str) -> std::io::Result<()> {
    let dir = fs::read_dir(dir_path)?;

    for entry in dir {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            match validate_json_file(&path) {
                Ok(_) => println!("{}: Valid JSON", path.display()),
                Err(e) => println!("{}: Not valid JSON - {}", path.display(), e),
            }
        }
    }

    Ok(())
}

fn validate_json_file(file_path: &Path) -> Result<(), String> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Error reading file: {}", e))?;

    serde_json::from_str::<Value>(&content)
        .map_err(|e| format!("Invalid JSON: {}", e))?;

    Ok(())
}
