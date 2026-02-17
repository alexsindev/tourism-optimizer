use crate::api_types::{BenchmarkRequest, BenchmarkResponse};
use crate::state::AppState;
use axum::{extract::State, http::StatusCode, Json};
use core::algorithms::{greedy_solve, sa_solve};
use std::sync::Arc;

#[utoipa::path(
    post,
    path = "/api/benchmark",
    request_body = BenchmarkRequest,
    responses(
        (status = 200, description = "Benchmark results", body = BenchmarkResponse),
        (status = 404, description = "Dataset not found")
    )
)]
pub async fn benchmark(
    State(state): State<Arc<AppState>>,
    Json(req): Json<BenchmarkRequest>,
) -> Result<Json<BenchmarkResponse>, StatusCode> {
    let attractions = state
        .datasets
        .get(&req.dataset.name)
        .ok_or(StatusCode::NOT_FOUND)?;

    let greedy = greedy_solve(attractions, &req.params, 42);
    let sa = sa_solve(attractions, &req.params, 42);

    Ok(Json(BenchmarkResponse {
        greedy,
        simulated_annealing: sa,
    }))
}
