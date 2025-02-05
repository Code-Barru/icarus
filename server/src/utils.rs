use crate::{live, state::GlobalState};
use axum::{Router, handler::HandlerWithoutStateExt};
use diesel::{Connection, PgConnection};
use http::StatusCode;
use std::env;
use std::net::SocketAddr;
use tower_http::{
    services::ServeDir,
    trace::{self, TraceLayer},
};
use tracing::{Level, info, warn};

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    match PgConnection::establish(&database_url) {
        Ok(conn) => {
            info!("Connected to database");
            conn
        }
        Err(e) => panic!("Error connecting to {}: {}", database_url, e),
    }
}

pub fn get_router(state: GlobalState) -> Router {
    Router::new()
        .nest("/agents", crate::agents::routes::get_router(&state))
        .nest("/tasks", crate::tasks::route::get_router(&state))
        .nest("/dist", get_serve_dir())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
}

pub fn get_serve_dir() -> Router {
    async fn handle_404() -> (StatusCode, &'static str) {
        (StatusCode::NOT_FOUND, "404 Not Found")
    }

    let service_404 = handle_404.into_service();

    let serve_dir = ServeDir::new("dist/").not_found_service(service_404);

    Router::new().fallback_service(serve_dir)
}

pub fn start_logger() {
    let mut env_set: bool = false;
    let level = match std::env::var("RUST_LOG") {
        Ok(val) => {
            env_set = true;
            val
        }
        Err(_) => "info".to_string(),
    };
    let level = match level.as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(level)
        .compact()
        .init();

    if !env_set {
        warn!("RUST_LOG not set, using default value 'info'");
    }
}

pub fn start_api_server(state: GlobalState) -> (tokio::task::JoinHandle<()>, SocketAddr) {
    let api_addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let api_server = crate::api::WebServer::new(state.clone());

    (
        tokio::spawn(async move {
            api_server.start_server(&api_addr).await;
        }),
        api_addr,
    )
}

pub fn start_rt_server(state: GlobalState) -> (tokio::task::JoinHandle<()>, SocketAddr) {
    let rt_addr = SocketAddr::from(([0, 0, 0, 0], 1337));
    let rt_server = live::RTServer::new(state);

    (
        tokio::spawn(async move {
            rt_server.start_server(&rt_addr).await;
        }),
        rt_addr,
    )
}
