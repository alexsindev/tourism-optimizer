use crate::state::AppState;
use crate::api_types::DatasetInfo;
use axum::{extract::State, Json};
use std::sync::Arc;

#[utoipa::path(
    get,
    path = "/api/datasets",
    responses(
        (status = 200, description = "List of available datasets", body = Vec<DatasetInfo>)
    )
)]
pub async fn list_datasets(State(state): State<Arc<AppState>>) -> Json<Vec<DatasetInfo>> {
    let datasets: Vec<DatasetInfo> = state
        .datasets
        .iter()
        .map(|(name, attractions)| DatasetInfo {
            name: name.clone(),
            size: attractions.len(),
        })
        .collect();
    
    Json(datasets)
}
