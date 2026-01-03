use serde::{Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct JobResponse {
    pub id: Uuid,
    pub status: String,
    pub task_type: Option<String>,
    pub payload: Option<String>,
    pub result: Option<String>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}
