use std::net::SocketAddr;

use tracing::info;

mod agents;
mod api;
mod live;
mod schema;
pub mod state;
pub mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    info!("Starting Icarus Servers...\n");
    let state = state::GlobalState::new();

    let api_addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let api_server = api::WebServer::new(state.clone());

    let api_server_handle = tokio::spawn(async move {
        api_server.start_server(&api_addr).await;
    });
    info!("API Server listening at : {}", api_addr);

    let rt_addr = SocketAddr::from(([0, 0, 0, 0], 1337));
    let rt_server = live::RTServer::new(state);

    let rt_server_handle = tokio::spawn(async move {
        rt_server.start_server(&rt_addr).await;
    });
    info!("RT  Server listening at : {}", rt_addr);

    match tokio::try_join!(api_server_handle, rt_server_handle) {
        Ok(_) => (),
        Err(e) => info!("Error starting servers: {:?}", e),
    };
}
