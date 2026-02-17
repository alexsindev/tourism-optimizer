use core::models::itinerary::Itinerary;
use core::models::constraints::SolveParams;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct SolveRequest {
    pub algorithm: String,
    pub dataset: DatasetSpec,
    pub params: SolveParams,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct DatasetSpec {
    #[serde(rename = "type")]
    #[allow(dead_code)]
    pub dataset_type: String,
    pub name: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SolveResponse {
    pub itinerary: Itinerary,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct BenchmarkRequest {
    pub dataset: DatasetSpec,
    pub params: SolveParams,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BenchmarkResponse {
    pub greedy: Itinerary,
    pub simulated_annealing: Itinerary,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ExperimentRequest {
    pub dataset: String,
    pub n_runs: usize,
    pub params: SolveParams,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ExperimentResponse {
    pub dataset_name: String,
    pub dataset_size: usize,
    pub greedy: AlgorithmStats,
    pub sa: AlgorithmStats,
    pub sa_improvement_pct: f64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AlgorithmStats {
    pub mean_satisfaction: f64,
    pub std_satisfaction: f64,
    pub mean_ms: f64,
    pub valid_runs: usize,
    pub mean_attractions: f64,
    pub mean_cost: f64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DatasetInfo {
    pub name: String,
    pub size: usize,
}
