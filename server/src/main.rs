use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

mod agents;
mod tasks;
mod utils;
mod ws;

use agents::models::AgentEntry;
use axum::Router;
use tasks::models::TaskEntry;

use http::{header, Method};
use socketioxide::SocketIo;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

static AGENTS_HEALTH_CHECK_INTERVAL: u64 = 1;
static AGENTS_HEALTH_CHECK_TIMEOUT: u64 = 10;

#[derive(Clone)]
struct AppState {
    agents: Arc<Mutex<Vec<AgentEntry>>>,
    tasks: Arc<Mutex<Vec<TaskEntry>>>,
    io: SocketIo,
}

#[tokio::main]
async fn main() {
    let (layer, io) = SocketIo::new_layer();
    ws::services::setup_ws(&io);

    let state = AppState {
        agents: Arc::new(Mutex::new(Vec::new())),
        tasks: Arc::new(Mutex::new(Vec::new())),
        io,
    };

    // setup logger
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any)
        .allow_headers(vec![
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::ACCEPT,
        ]);

    // setup router & services
    let app = Router::new()
        .nest("/agents", agents::services::get_router(state.clone()))
        .nest("/tasks", tasks::services::get_router(state.clone()))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(cors)
        .fallback(utils::not_found_handler)
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(layer),
        );

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
        let mut io_clone = state.io.clone();
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(AGENTS_HEALTH_CHECK_INTERVAL)).await;
            utils::agents_health_check(&mut agents_clone, &mut io_clone).await;
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
