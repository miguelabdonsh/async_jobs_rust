use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use uuid::Uuid;
use crate::dto::responses::{JobResponse, ErrorResponse};


pub async fn create_job(
    State(state): State<crate::models::jobs::AppState>,
    Json(input): Json<crate::dto::requests::CreateJobRequest>,
) -> impl IntoResponse {
    let job_id = Uuid::new_v4();

    let job = crate::models::jobs::Job {
        id: job_id,
        status: "Pending".to_string(),
        task_type: input.task_type,
        payload: input.payload,
        result: None,
    };

    state.jobs.lock().await.insert(job_id, job.clone());
    let _ = state.job_sender.send(job_id).await;

    let response = JobResponse {
        id: job.id,
        status: job.status,
        task_type: Some(job.task_type),
        payload: Some(job.payload),
        result: job.result,
    };

    return Json(response);
}


pub async fn get_job_status(
    Path(job_id): Path<Uuid>,
    State(state): State<crate::models::jobs::AppState>,
) -> impl IntoResponse {
    let jobs = state.jobs.lock().await;

    if let Some(job) = jobs.get(&job_id) {
        let response = JobResponse {
            id: job.id,
            status: job.status.clone(),
            task_type: Some(job.task_type.clone()),
            payload: Some(job.payload.clone()),
            result: job.result.clone(),
        };
        return Json(response).into_response();
    } else {
        let error_response = ErrorResponse {
            error: format!("Job with id {} not found", job_id),
        };
        return (axum::http::StatusCode::NOT_FOUND, Json(error_response)).into_response();
    }
}

pub async fn list_jobs(
    State(state): State<crate::models::jobs::AppState>,
) -> impl IntoResponse {
    let jobs = state.jobs.lock().await;

    let response: Vec<JobResponse> = jobs.values().map(|job| JobResponse {
        id: job.id,
        status: job.status.clone(),
        task_type: Some(job.task_type.clone()),
        payload: Some(job.payload.clone()),
        result: job.result.clone(),
    }).collect();

    return Json(response);
}
