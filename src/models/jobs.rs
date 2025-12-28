use uuid::Uuid;
use axum::{response::IntoResponse, Json};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use axum::extract::{State, Path};


#[derive(Clone)]
pub struct AppState{
    pub jobs: Arc<Mutex<HashMap<Uuid, String>>>,
}


#[derive(Serialize)]
struct JobResponse {
    id: Uuid,
    status: String,
}

pub async fn create_job(State(state): State<AppState>) -> impl IntoResponse {
    let job_id = Uuid::new_v4();
    let response = JobResponse {
        id: job_id,
        status: "Pending".to_string(),
    };

    state.jobs.lock().await.insert(job_id, "Pending".to_string());

    return Json(response);
}

pub async fn get_job_status(Path(job_id): Path<Uuid>, State(state): State<AppState>) -> impl IntoResponse {
    let jobs = state.jobs.lock().await;
    if let Some(job) = jobs.get(&job_id){
        let response = JobResponse {
            id: job_id,
            status: job.clone(),
        };
        return Json(response).into_response();
    } else {
        return (axum::http::StatusCode::NOT_FOUND, "Job not found").into_response();
    }

}