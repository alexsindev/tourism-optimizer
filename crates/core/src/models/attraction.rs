use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Category {
    Museum,
    Restaurant,
    Landmark,
    Park,
    Shopping,
    Entertainment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attraction {
    pub id: u32,
    pub name: String,
    pub location: Location,
    pub open_time: u32,   // minutes from midnight, e.g. 540 = 9:00 AM
    pub close_time: u32,  // e.g. 1080 = 6:00 PM
    pub duration: u32,    // expected visit duration in minutes
    pub fee: f64,         // entrance fee in USD
    pub preference: f64,  // user preference score 0.0 - 1.0
    pub category: Category,
}

impl Attraction {
    pub fn is_open_at(&self, time: u32) -> bool {
        time >= self.open_time && time + self.duration <= self.close_time
    }

    pub fn is_open_during(&self, start: u32, end: u32) -> bool {
        start >= self.open_time && end <= self.close_time
    }
}