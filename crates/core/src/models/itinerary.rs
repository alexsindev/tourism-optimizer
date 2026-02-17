use serde::{Deserialize, Serialize};

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct Visit {
    pub attraction_id: u32,
    pub attraction_name: String,
    pub arrival_time: u32,
    pub departure_time: u32,
    pub fee: f64,
    pub preference: f64,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct DayPlan {
    pub day: u32,
    pub visits: Vec<Visit>,
    pub total_travel_time: u32,
    pub total_cost: f64,
    pub total_satisfaction: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct Itinerary {
    pub days: Vec<DayPlan>,
    pub total_satisfaction: f64,
    pub total_cost: f64,
    pub total_attractions: usize,
    pub algorithm_used: String,
    pub computation_ms: u128,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convergence_data: Option<Vec<ConvergencePoint>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct ConvergencePoint {
    pub iteration: u32,
    pub satisfaction: f64,
    pub temperature: f64,
}

impl Itinerary {
    pub fn empty(algorithm: String) -> Self {
        Self {
            days: Vec::new(),
            total_satisfaction: 0.0,
            total_cost: 0.0,
            total_attractions: 0,
            algorithm_used: algorithm,
            computation_ms: 0,
            convergence_data: None,
        }
    }

    pub fn compute_totals(&mut self) {
        self.total_satisfaction = self.days.iter().map(|d| d.total_satisfaction).sum();
        self.total_cost = self.days.iter().map(|d| d.total_cost).sum();
        self.total_attractions = self.days.iter().map(|d| d.visits.len()).sum();
    }
}