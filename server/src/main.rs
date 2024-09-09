use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

mod agents;
mod tasks;
mod utils;
mod web;

use agents::models::AgentEntry;
use axum::Router;
use tokio::net::TcpListener;
use tower_http::{
    services::ServeDir,
    trace::{self, TraceLayer},
};
use tracing::Level;

static AGENTS_HEALTH_CHECK_INTERVAL: u64 = 1;
static AGENTS_HEALTH_CHECK_TIMEOUT: u64 = 10;

#[derive(Clone)]
struct AppState {
    agents: Arc<Mutex<Vec<AgentEntry>>>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        agents: Arc::new(Mutex::new(Vec::new())),
    };

    // setup logger
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    // setup router & services
    let app = Router::new()
        .nest_service("/assets", ServeDir::new("templates/assets"))
        .nest("/", web::services::get_router(state.clone()))
        .nest("/c2/agents", agents::services::get_router(state.clone()))
        .nest("/c2/tasks", tasks::services::get_router(state.clone()))
        .route("/favicon.ico", axum::routing::get(utils::get_favicon))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .fallback(utils::not_found_handler);

    // setup server
    let listener = match TcpListener::bind("0.0.0.0:1337").await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind to port 1337: {}", e);
            return;
        }
    };

    // setup agents health check
    tokio::spawn(async move {
        let mut agents_clone = state.agents.clone();
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(AGENTS_HEALTH_CHECK_INTERVAL)).await;
            utils::agents_health_check(&mut agents_clone).await;
        }
    });

    // start server
    match axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    {
        Ok(_) => println!("Server exited successfully"),
        Err(e) => eprintln!("Server exited with an error: {}", e),
    };
}
