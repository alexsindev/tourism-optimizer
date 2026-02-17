mod api_types;
mod routes;
mod state;

use axum::{
    routing::{get, post},
    Router,
};
use state::AppState;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::datasets::list_datasets,
        routes::solve::solve,
        routes::benchmark::benchmark,
        routes::experiment::experiment,
    ),
    components(schemas(
        api_types::SolveRequest,
        api_types::SolveResponse,
        api_types::BenchmarkRequest,
        api_types::BenchmarkResponse,
        api_types::ExperimentRequest,
        api_types::ExperimentResponse,
        api_types::DatasetSpec,
        api_types::AlgorithmStats,
        api_types::DatasetInfo,
        core::models::constraints::SolveParams,
        core::models::itinerary::Itinerary,
        core::models::itinerary::DayPlan,
        core::models::itinerary::Visit,
        core::models::itinerary::ConvergencePoint,
    ))
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState::new());

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/api/datasets", get(routes::datasets::list_datasets))
        .route("/api/solve", post(routes::solve::solve))
        .route("/api/benchmark", post(routes::benchmark::benchmark))
        .route("/api/experiment", post(routes::experiment::experiment))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    println!("🚀 Server running at http://localhost:3000");
    println!("📚 API docs at http://localhost:3000/docs");
    
    axum::serve(listener, app).await.unwrap();
}
