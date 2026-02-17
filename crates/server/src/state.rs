use core::models::attraction::Attraction;
use std::collections::HashMap;
use std::fs;
use std::env;

pub struct AppState {
    pub datasets: HashMap<String, Vec<Attraction>>,
}

impl AppState {
    pub fn new() -> Self {
        let mut datasets = HashMap::new();
        
        // Get the project root - go up from the executable location
        let dataset_dir = env::current_dir()
            .expect("Failed to get current directory")
            .join("data")
            .join("datasets");
        
        println!("Looking for datasets in: {}", dataset_dir.display());
        
        if !dataset_dir.exists() {
            eprintln!("❌ Dataset directory does not exist: {}", dataset_dir.display());
            eprintln!("Run from project root: cargo run -p server");
            return Self { datasets: HashMap::new() };
        }
        
        // Load all JSON files from datasets directory
        match fs::read_dir(&dataset_dir) {
            Ok(entries) => {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("json") {
                        let name = path.file_stem()
                            .and_then(|s| s.to_str())
                            .unwrap_or("unknown")
                            .to_string();
                        
                        println!("Trying to load: {}", path.display());
                        
                        match fs::read_to_string(&path) {
                            Ok(content) => {
                                match serde_json::from_str::<Vec<Attraction>>(&content) {
                                    Ok(attractions) => {
                                        println!("✓ Loaded dataset: {} ({} attractions)", name, attractions.len());
                                        datasets.insert(name, attractions);
                                    }
                                    Err(e) => {
                                        eprintln!("❌ Failed to parse {}: {}", path.display(), e);
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("❌ Failed to read {}: {}", path.display(), e);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Failed to read directory {}: {}", dataset_dir.display(), e);
            }
        }
        
        println!("Total datasets loaded: {}", datasets.len());
        
        Self { datasets }
    }
}
