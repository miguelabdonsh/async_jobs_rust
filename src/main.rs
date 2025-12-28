mod routes;
mod models;


#[tokio::main]
async fn main() {
    let state = models::jobs::AppState {
        jobs: std::sync::Arc::new(tokio::sync::Mutex::new(std::collections::HashMap::new())),
    };
    let app = routes::create_router(state);
    

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}