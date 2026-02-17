use crate::api_types::{AlgorithmStats, ExperimentRequest, ExperimentResponse};
use crate::state::AppState;
use axum::{extract::State, http::StatusCode, Json};
use core::algorithms::{greedy_solve, sa_solve};
use core::utils::experiments::run_experiments;
use std::sync::Arc;

#[utoipa::path(
    post,
    path = "/api/experiment",
    request_body = ExperimentRequest,
    responses(
        (status = 200, description = "Experiment results", body = ExperimentResponse),
        (status = 404, description = "Dataset not found")
    )
)]
pub async fn experiment(
    State(state): State<Arc<AppState>>,
    Json(req): Json<ExperimentRequest>,
) -> Result<Json<ExperimentResponse>, StatusCode> {
    let attractions = state
        .datasets
        .get(&req.dataset)
        .ok_or(StatusCode::NOT_FOUND)?;

    let greedy_stats = run_experiments(
        attractions,
        &req.params,
        req.n_runs,
        |attrs, params, seed| greedy_solve(attrs, params, seed),
    );

    let sa_stats = run_experiments(
        attractions,
        &req.params,
        req.n_runs,
        |attrs, params, seed| sa_solve(attrs, params, seed),
    );

    let improvement = if greedy_stats.mean_satisfaction > 0.0 {
        ((sa_stats.mean_satisfaction - greedy_stats.mean_satisfaction) / greedy_stats.mean_satisfaction) * 100.0
    } else {
        0.0
    };

    Ok(Json(ExperimentResponse {
        dataset_name: req.dataset,
        dataset_size: attractions.len(),
        greedy: AlgorithmStats {
            mean_satisfaction: greedy_stats.mean_satisfaction,
            std_satisfaction: greedy_stats.std_satisfaction,
            mean_ms: greedy_stats.mean_ms,
            valid_runs: greedy_stats.valid_runs,
            mean_attractions: greedy_stats.mean_attractions,
            mean_cost: greedy_stats.mean_cost,
        },
        sa: AlgorithmStats {
            mean_satisfaction: sa_stats.mean_satisfaction,
            std_satisfaction: sa_stats.std_satisfaction,
            mean_ms: sa_stats.mean_ms,
            valid_runs: sa_stats.valid_runs,
            mean_attractions: sa_stats.mean_attractions,
            mean_cost: sa_stats.mean_cost,
        },
        sa_improvement_pct: improvement,
    }))
}
