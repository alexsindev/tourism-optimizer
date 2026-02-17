use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Visit {
    pub attraction_id: u32,
    pub arrival_time: u32,   // minutes from midnight
    pub departure_time: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayPlan {
    pub day: u32,
    pub visits: Vec<Visit>,
    pub total_travel_time: u32,
    pub total_cost: f64,
    pub total_satisfaction: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Itinerary {
    pub days: Vec<DayPlan>,
    pub total_satisfaction: f64,
    pub total_cost: f64,
    pub total_attractions: usize,
    pub algorithm_used: String,
    pub computation_ms: u128,  // how long the algorithm took
}

impl Itinerary {
    pub fn new(algorithm_used: &str) -> Self {
        Itinerary {
            days: Vec::new(),
            total_satisfaction: 0.0,
            total_cost: 0.0,
            total_attractions: 0,
            algorithm_used: algorithm_used.to_string(),
            computation_ms: 0,
        }
    }

    pub fn add_day(&mut self, day: DayPlan) {
        self.total_satisfaction += day.total_satisfaction;
        self.total_cost += day.total_cost;
        self.total_attractions += day.visits.len();
        self.days.push(day);
    }
}