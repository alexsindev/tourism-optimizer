use core::models::attraction::Attraction;
use std::collections::HashMap;
use std::fs;

pub struct AppState {
    pub datasets: HashMap<String, Vec<Attraction>>,
}

impl AppState {
    pub fn new() -> Self {
        let mut datasets = HashMap::new();
        
        // Load built-in datasets
        for name in &["small", "medium", "large"] {
            let path = format!("data/datasets/{}.json", name);
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(attractions) = serde_json::from_str::<Vec<Attraction>>(&content) {
                    println!("✓ Loaded dataset: {} ({} attractions)", name, attractions.len());
                    datasets.insert(name.to_string(), attractions);
                }
            }
        }
        
        Self { datasets }
    }
}
