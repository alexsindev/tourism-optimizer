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
        }
    }
}