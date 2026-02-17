# Real Data & Interactive Map Features

## 🗺️ Interactive Pin Dropping

You can now set custom start and end points by clicking directly on the map!

### How to Use

1. **Set Start Point:**
   - Click the "📍 Set" button next to "Start Point" in the control panel
   - Click anywhere on the map to place your start point
   - The gold marker will move to your selected location

2. **Set End Point (Optional):**
   - Click the "🏁 Set" button next to "End Point"
   - Click anywhere on the map to place your end point
   - A red marker will appear at your selected location
   - If not set, the algorithm will return to the start point

3. **Cancel Pin Dropping:**
   - Click the "Cancel" button or simply solve with current settings

4. **Clear End Point:**
   - Click "Clear end point" to make start and end the same location

### Use Cases

- **Hotel to Airport:** Start at your hotel, end at the airport
- **Different Hotels:** Start at one hotel, end at another
- **Custom Tours:** Create tours starting from any location
- **Flexible Planning:** Adjust based on your actual travel plans

---

## 🌏 Real Bangkok Dataset

Fetch real attraction data from OpenStreetMap!

### Fetching Real Data

Run the data fetcher:

```bash
cargo run -p fetch_real_data
```

This will:

1. Query OpenStreetMap's Overpass API for Bangkok attractions
2. Fetch museums, galleries, viewpoints, restaurants, cafes, and attractions
3. Generate realistic parameters (opening hours, fees, durations)
4. Save to `data/datasets/bangkok_real.json`

### What Gets Fetched

- **Tourism attractions:** Museums, galleries, viewpoints, landmarks
- **Restaurants & Cafes:** Real dining options
- **Radius:** 10km around Bangkok center (13.7563°N, 100.5018°E)
- **Limit:** Up to 200 attractions to keep dataset manageable

### Generated Parameters

Since OpenStreetMap doesn't include all our required fields, we generate
realistic defaults:

| Category      | Open Time | Close Time | Duration | Fee  |
| ------------- | --------- | ---------- | -------- | ---- |
| Museum        | 9:00      | 17:00      | 90 min   | $10  |
| Restaurant    | 11:00     | 22:00      | 60 min   | $15  |
| Landmark      | 6:00      | 18:00      | 45 min   | $5   |
| Park          | 6:00      | 18:00      | 90 min   | Free |
| Shopping      | 10:00     | 21:00      | 120 min  | $20  |
| Entertainment | 12:00     | 23:00      | 120 min  | $25  |

**Preferences** are randomized between 0.5-1.0 for variety.

### Loading Real Data

After fetching, the server will automatically load `bangkok_real.json`:

```bash
# Restart the server to reload datasets
cargo run -p server
```

You should see:

```
✓ Loaded dataset: bangkok_real (150 attractions)
```

Then select "bangkok_real" from the dataset dropdown in the UI!

---

## 🔌 Using Your Own Data Sources

### Option 1: Google Places API

If you have a Google Places API key, you can enhance the fetcher:

1. Add to `fetch_real_data/Cargo.toml`:

```toml
reqwest = { version = "0.11", features = ["json"] }
```

2. Modify the fetcher to call Google Places API
3. Extract full details: ratings, reviews, actual hours, photos

### Option 2: Custom JSON

Create your own `custom.json` in `data/datasets/`:

```json
[
  {
    "id": 1,
    "name": "Grand Palace",
    "location": { "lat": 13.75, "lng": 100.4915 },
    "open_time": 510,
    "close_time": 930,
    "duration": 120,
    "fee": 17.5,
    "preference": 0.95,
    "category": "Landmark"
  }
]
```

Restart the server and it will load automatically!

### Option 3: CSV Import

Convert CSV data to JSON format:

```bash
# Example: convert from CSV
python3 << 'EOF'
import csv, json

with open('attractions.csv') as f:
    reader = csv.DictReader(f)
    data = [{
        "id": int(row['id']),
        "name": row['name'],
        "location": {"lat": float(row['lat']), "lng": float(row['lng'])},
        "open_time": int(row['open_time']),
        "close_time": int(row['close_time']),
        "duration": int(row['duration']),
        "fee": float(row['fee']),
        "preference": float(row['preference']),
        "category": row['category']
    } for row in reader]

with open('data/datasets/imported.json', 'w') as f:
    json.dump(data, f, indent=2)
EOF
```

---

## 📝 Time Format Reference

Times are in **minutes from midnight**:

| Time     | Minutes |
| -------- | ------- |
| 6:00 AM  | 360     |
| 8:00 AM  | 480     |
| 9:00 AM  | 540     |
| 12:00 PM | 720     |
| 5:00 PM  | 1020    |
| 6:00 PM  | 1080    |
| 9:00 PM  | 1260    |
| 11:00 PM | 1380    |

---

## 🚀 Complete Workflow

### Full Setup with Real Data

```bash
# 1. Generate synthetic datasets
cargo run -p generate

# 2. Fetch real Bangkok data
cargo run -p fetch_real_data

# 3. Start server (loads all datasets)
cargo run -p server

# 4. Start frontend
cd frontend
npm run dev

# 5. Open http://localhost:5173
# 6. Select "bangkok_real" from dataset dropdown
# 7. Click map to set start/end points
# 8. Click "Solve" to optimize your route!
```

### Tips

- **Real coordinates work best** - The algorithms use actual distances
- **Set realistic budgets** - Real Bangkok attractions vary in cost
- **Adjust time budgets** - Account for Bangkok traffic (use longer travel
  times)
- **Save your pins** - The coordinates are shown in the UI

---

## 🎯 Advanced: Integrating Other Cities

To adapt for other cities:

1. Change `BANGKOK_CENTER_LAT` and `BANGKOK_CENTER_LNG` in
   `fetch_real_data/src/main.rs`
2. Adjust `RADIUS_KM` for city size
3. Run the fetcher: `cargo run -p fetch_real_data`
4. Restart server to load new data

Example for Tokyo:

```rust
const TOKYO_CENTER_LAT: f64 = 35.6762;
const TOKYO_CENTER_LNG: f64 = 139.6503;
```

Example for Paris:

```rust
const PARIS_CENTER_LAT: f64 = 48.8566;
const PARIS_CENTER_LNG: f64 = 2.3522;
```

Enjoy your custom tourism optimizer! 🌍✨
