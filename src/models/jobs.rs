use uuid::Uuid;
use axum::{response::IntoResponse, Json};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use axum::extract::{State, Path};

#[derive(Serialize)]
pub struct JobResponse {
    pub id: Uuid,
    pub status: String,
    pub task_type: Option<String>,
    pub payload: Option<String>,
}


#[derive(Deserialize)]
pub struct CreateJobRequest {
    pub task_type: String,
    pub payload: String,
}

#[derive(Clone)]
pub struct Job {
    pub id: Uuid,
    pub status: String,
    pub task_type: String,
    pub payload: String,
}

#[derive(Clone)]
pub struct AppState{
    pub jobs: Arc<Mutex<HashMap<Uuid, Job>>>,
}

pub async fn create_job(
    State(state): State<AppState>,
    Json(input): Json<CreateJobRequest>,
) -> impl IntoResponse {
    let job_id = Uuid::new_v4();

    let job = Job {
        id: job_id,
        status: "Pending".to_string(),
        task_type: input.task_type,
        payload: input.payload,
    };

    state.jobs.lock().await.insert(job_id, job.clone());

    let response = JobResponse {
        id: job.id,
        status: job.status,
        task_type: Some(job.task_type),
        payload: Some(job.payload),
    };

    return Json(response);
}


pub async fn get_job_status(
    Path(job_id): Path<Uuid>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let jobs = state.jobs.lock().await;

    if let Some(job) = jobs.get(&job_id) {
        let response = JobResponse {
            id: job.id,
            status: job.status.clone(),
            task_type: Some(job.task_type.clone()),
            payload: Some(job.payload.clone()),
        };

        return Json(response).into_response();
    } else {
        return (axum::http::StatusCode::NOT_FOUND, "Job not found").into_response();
    }
}
