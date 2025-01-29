use std::net::SocketAddr;

use tokio::net::TcpListener;
use tracing::info;

mod agents;
mod schema;
pub mod state;
pub mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let state = state::GlobalState::new();

    let app = utils::get_router(state);

    let listener = match TcpListener::bind("0.0.0.0:1337").await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind: {}", e);
            return;
        }
    };

    info!("Server started at: {}", listener.local_addr().unwrap());
    match axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Server error: {}", e);
        }
    }
}
