use serde::{Deserialize, Serialize};

#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct SolveParams {
    pub num_days: u32,
    pub daily_time_budget: u32,  // minutes available per day
    pub total_budget: f64,        // total money budget in USD
    pub start_time: u32,          // daily start time in minutes from midnight
    pub hotel_lat: f64,
    pub hotel_lng: f64,
    #[serde(default)]
    pub end_lat: Option<f64>,     // optional different end location
    #[serde(default)]
    pub end_lng: Option<f64>,     // optional different end location
}

impl Default for SolveParams {
    fn default() -> Self {
        Self {
            num_days: 2,
            daily_time_budget: 600,  // 10 hours
            total_budget: 100.0,
            start_time: 540,         // 09:00
            hotel_lat: 13.7563,
            hotel_lng: 100.5018,
            end_lat: None,
            end_lng: None,
        }
    }
}