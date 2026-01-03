mod routes;
mod models;
mod handlers;
mod dto;
mod workers;

#[tokio::main]
async fn main() {
    let jobs = std::sync::Arc::new(tokio::sync::Mutex::new(std::collections::HashMap::new()));
    let (sender, receiver) = tokio::sync::mpsc::channel(100);

    workers::job_worker::start_worker(jobs.clone(), receiver);

    let state = models::jobs::AppState { jobs, job_sender: sender };
    let app = routes::create_router(state);
    

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
