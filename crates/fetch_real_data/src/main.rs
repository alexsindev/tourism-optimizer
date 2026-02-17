//! Fetch real Bangkok attraction data from OpenStreetMap Overpass API

use core::models::attraction::{Attraction, Category, Location};
use serde::Deserialize;
use std::fs;

const BANGKOK_CENTER_LAT: f64 = 13.7563;
const BANGKOK_CENTER_LNG: f64 = 100.5018;
const RADIUS_KM: f64 = 10.0; // 10km radius around center

#[derive(Debug, Deserialize)]
struct OverpassResponse {
    elements: Vec<OverpassElement>,
}

#[derive(Debug, Deserialize)]
struct OverpassElement {
    #[serde(default)]
    #[allow(dead_code)]
    id: u64,
    #[serde(default)]
    lat: f64,
    #[serde(default)]
    lon: f64,
    tags: Option<Tags>,
}

#[derive(Debug, Deserialize)]
struct Tags {
    name: Option<String>,
    tourism: Option<String>,
    amenity: Option<String>,
    #[serde(rename = "opening_hours")]
    #[allow(dead_code)]
    opening_hours: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌏 Fetching real Bangkok attraction data from OpenStreetMap...");
    
    // Overpass API query for tourism attractions in Bangkok
    let query = format!(
        r#"[out:json][timeout:30];
        (
          node["tourism"~"attraction|museum|gallery|viewpoint"](around:{},{},{});
          node["amenity"~"restaurant|cafe"](around:{},{},{});
        );
        out body;
        "#,
        RADIUS_KM * 1000.0, BANGKOK_CENTER_LAT, BANGKOK_CENTER_LNG,
        RADIUS_KM * 1000.0, BANGKOK_CENTER_LAT, BANGKOK_CENTER_LNG
    );

    let client = reqwest::Client::new();
    let url = "https://overpass-api.de/api/interpreter";
    
    println!("📡 Querying Overpass API...");
    let response = client
        .post(url)
        .body(query)
        .send()
        .await?;

    if !response.status().is_success() {
        eprintln!("❌ API request failed: {}", response.status());
        return Ok(());
    }

    let overpass_data: OverpassResponse = response.json().await?;
    println!("✓ Received {} elements", overpass_data.elements.len());

    // Convert to our Attraction format
    let mut attractions: Vec<Attraction> = Vec::new();
    let mut id_counter = 1u32;

    for element in overpass_data.elements {
        if let Some(tags) = element.tags {
            if let Some(ref name) = tags.name {
                // Skip if no valid coordinates
                if element.lat == 0.0 || element.lon == 0.0 {
                    continue;
                }

                // Determine category
                let category = determine_category(&tags);
                
                // Generate realistic parameters
                let (open_time, close_time, duration, fee) = generate_params(&category);
                
                let preference = 0.5 + (id_counter as f64 * 0.013) % 0.5; // Spread between 0.5-1.0

                attractions.push(Attraction {
                    id: id_counter,
                    name: name.clone(),
                    location: Location {
                        lat: element.lat,
                        lng: element.lon,
                    },
                    open_time,
                    close_time,
                    duration,
                    fee,
                    preference,
                    category,
                });

                id_counter += 1;

                // Limit to prevent overwhelming dataset
                if attractions.len() >= 200 {
                    break;
                }
            }
        }
    }

    println!("✓ Processed {} attractions", attractions.len());

    // Save to file
    fs::create_dir_all("data/datasets")?;
    let json = serde_json::to_string_pretty(&attractions)?;
    fs::write("data/datasets/bangkok_real.json", json)?;

    println!("✅ Saved to data/datasets/bangkok_real.json");
    println!();
    println!("Summary:");
    println!("  - Total attractions: {}", attractions.len());
    
    let by_category: std::collections::HashMap<String, usize> = attractions
        .iter()
        .fold(std::collections::HashMap::new(), |mut acc, a| {
            *acc.entry(format!("{:?}", a.category)).or_insert(0) += 1;
            acc
        });
    
    for (cat, count) in by_category {
        println!("  - {}: {}", cat, count);
    }

    Ok(())
}

fn determine_category(tags: &Tags) -> Category {
    if let Some(tourism) = &tags.tourism {
        match tourism.as_str() {
            "museum" | "gallery" => Category::Museum,
            "attraction" | "viewpoint" => Category::Landmark,
            _ => Category::Entertainment,
        }
    } else if let Some(amenity) = &tags.amenity {
        match amenity.as_str() {
            "restaurant" | "cafe" => Category::Restaurant,
            _ => Category::Entertainment,
        }
    } else {
        Category::Landmark
    }
}

fn generate_params(category: &Category) -> (u32, u32, u32, f64) {
    match category {
        Category::Museum => (540, 1020, 90, 10.0),      // 9-17, 90min, $10
        Category::Restaurant => (660, 1320, 60, 15.0),  // 11-22, 60min, $15
        Category::Landmark => (360, 1080, 45, 5.0),     // 6-18, 45min, $5
        Category::Park => (360, 1080, 90, 0.0),         // 6-18, 90min, free
        Category::Shopping => (600, 1260, 120, 20.0),   // 10-21, 120min, $20
        Category::Entertainment => (720, 1380, 120, 25.0), // 12-23, 120min, $25
    }
}
