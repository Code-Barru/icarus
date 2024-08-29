use std::sync::{Arc, Mutex};

use axum::Router;
mod agents;
use agents::models::AgentEntry;
use tokio::net::TcpListener;

mod tasks;

#[derive(Clone)]
struct AppState {
    agents: Arc<Mutex<Vec<AgentEntry>>>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        agents: Arc::new(Mutex::new(Vec::new())),
    };

    let app = Router::new()
        .nest("/agents", agents::services::get_router(state.clone()))
        .nest("/tasks", tasks::services::get_router(state.clone()));

    let listener = match TcpListener::bind("0.0.0.0:8080").await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind to port 8080: {}", e);
            return;
        }
    };

    match axum::serve(listener, app).await {
        Ok(_) => println!("Server exited successfully"),
        Err(e) => eprintln!("Server exited with an error: {}", e),
    };
}
