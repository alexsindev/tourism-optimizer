use crate::api_types::{SolveRequest, SolveResponse};
use crate::state::AppState;
use axum::{extract::State, http::StatusCode, Json};
use core::algorithms::{greedy_solve, sa_solve};
use std::sync::Arc;

#[utoipa::path(
    post,
    path = "/api/solve",
    request_body = SolveRequest,
    responses(
        (status = 200, description = "Solved itinerary", body = SolveResponse),
        (status = 400, description = "Invalid request"),
        (status = 404, description = "Dataset not found")
    )
)]
pub async fn solve(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SolveRequest>,
) -> Result<Json<SolveResponse>, StatusCode> {
    let attractions = state
        .datasets
        .get(&req.dataset.name)
        .ok_or(StatusCode::NOT_FOUND)?;

    let itinerary = match req.algorithm.as_str() {
        "greedy" => greedy_solve(attractions, &req.params, 42),
        "simulated_annealing" => sa_solve(attractions, &req.params, 42),
        _ => return Err(StatusCode::BAD_REQUEST),
    };

    Ok(Json(SolveResponse { itinerary }))
}
