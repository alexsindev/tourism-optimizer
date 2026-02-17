use core::models::attraction::{Attraction, Category, Location};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::fs;

const BANGKOK_LAT: f64 = 13.7563;
const BANGKOK_LNG: f64 = 100.5018;

struct DatasetConfig {
    name: &'static str,
    n_attractions: usize,
    coord_spread: f64,
}

fn main() {
    let configs = vec![
        DatasetConfig { name: "small", n_attractions: 15, coord_spread: 0.05 },
        DatasetConfig { name: "medium", n_attractions: 75, coord_spread: 0.10 },
        DatasetConfig { name: "large", n_attractions: 300, coord_spread: 0.15 },
    ];

    fs::create_dir_all("data/datasets").expect("Failed to create datasets directory");

    for config in configs {
        let attractions = generate_dataset(config.n_attractions, config.coord_spread);
        let json = serde_json::to_string_pretty(&attractions).unwrap();
        let path = format!("data/datasets/{}.json", config.name);
        fs::write(&path, json).expect("Failed to write dataset");
        println!("✓ Generated {} with {} attractions", config.name, attractions.len());
    }
}

fn generate_dataset(n: usize, spread: f64) -> Vec<Attraction> {
    let mut rng = StdRng::seed_from_u64(42);
    let categories = [
        Category::Museum,
        Category::Restaurant,
        Category::Landmark,
        Category::Park,
        Category::Shopping,
        Category::Entertainment,
    ];

    (0..n)
        .map(|i| {
            let category = categories[i % categories.len()];
            let (open, close, duration, fee) = match category {
                Category::Museum => (540, 1020, rng.gen_range(60..180), rng.gen_range(3.0..20.0)),
                Category::Restaurant => (660, 1320, rng.gen_range(45..90), rng.gen_range(10.0..40.0)),
                Category::Landmark => (360, 1080, rng.gen_range(30..90), rng.gen_range(0.0..15.0)),
                Category::Park => (360, 1080, rng.gen_range(60..120), 0.0),
                Category::Shopping => (600, 1260, rng.gen_range(60..180), rng.gen_range(5.0..50.0)),
                Category::Entertainment => (720, 1380, rng.gen_range(90..240), rng.gen_range(10.0..50.0)),
            };

            Attraction {
                id: i as u32 + 1,
                name: format!("{:?} {}", category, i + 1),
                location: Location {
                    lat: BANGKOK_LAT + rng.gen_range(-spread..spread),
                    lng: BANGKOK_LNG + rng.gen_range(-spread..spread),
                },
                open_time: open,
                close_time: close,
                duration,
                fee,
                preference: rng.gen_range(0.3..1.0),
                category,
            }
        })
        .collect()
}