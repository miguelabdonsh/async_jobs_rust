use axum::{Router, routing::get, routing::post};
use crate::models::{jobs::AppState};
use crate::handlers;
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(|| async { "{\"status\": \"ok\"}" }))
        .route("/jobs", post(handlers::job_handler::create_job))
        .route("/jobs", get(handlers::job_handler::list_jobs))
        .route("/jobs/{job_id}", get(handlers::job_handler::get_job_status))
        .with_state(state)
}
