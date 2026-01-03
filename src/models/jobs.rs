use uuid::Uuid;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Job {
    pub id: Uuid,
    pub status: String,
    pub task_type: String,
    pub payload: String,
    pub result: Option<String>,
}

#[derive(Clone)]
pub struct AppState {
    pub jobs: Arc<Mutex<HashMap<Uuid, Job>>>,
    pub job_sender: tokio::sync::mpsc::Sender<Uuid>,
}

