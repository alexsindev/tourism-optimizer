use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Category {
    Museum,
    Restaurant,
    Landmark,
    Park,
    Shopping,
    Entertainment,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Location {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attraction {
    pub id: u32,
    pub name: String,
    pub location: Location,
    pub open_time: u32,      // minutes from midnight
    pub close_time: u32,     // minutes from midnight
    pub duration: u32,       // expected visit time in minutes
    pub fee: f64,            // entrance fee in USD
    pub preference: f64,     // user preference score ∈ [0.0, 1.0]
    pub category: Category,
}

impl Attraction {
    pub fn is_open_at(&self, time: u32) -> bool {
        time >= self.open_time && time < self.close_time
    }

    pub fn can_visit_at(&self, arrival_time: u32) -> bool {
        let departure_time = arrival_time + self.duration;
        arrival_time >= self.open_time && departure_time <= self.close_time
    }
}