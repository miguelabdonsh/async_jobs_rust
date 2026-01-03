use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::{mpsc, Mutex};
use tokio::time::{sleep, Duration};
use uuid::Uuid;
use crate::models::jobs::Job;

type JobStore = Arc<Mutex<HashMap<Uuid, Job>>>;

pub fn start_worker(jobs: JobStore, mut receiver: mpsc::Receiver<Uuid>) {
    tokio::spawn(async move {
        while let Some(job_id) = receiver.recv().await {
            let jobs = jobs.clone();
            tokio::spawn(async move {
                // Mark as Running
                if let Some(job) = jobs.lock().await.get_mut(&job_id) {
                    job.status = "Running".to_string();
                }

                // Simulate work (2 seconds)
                sleep(Duration::from_secs(2)).await;

                // Mark as Completed
                if let Some(job) = jobs.lock().await.get_mut(&job_id) {
                    job.status = "Completed".to_string();
                    job.result = Some(format!("Job {} done", job_id));
                }
            });
        }
    });
}
