use axum::{Router, routing::get, routing::post};
use crate::models::{self, jobs::AppState};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(|| async { "{\"status\": \"ok\"}" }))
        .route("/jobs", post(models::jobs::create_job))
        .route("/jobs/{job_id}", get(models::jobs::get_job_status))
        .with_state(state)
}