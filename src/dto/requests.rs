use serde::{Deserialize};


#[derive(Deserialize)]
pub struct CreateJobRequest {
    pub task_type: String,
    pub payload: String,
}
