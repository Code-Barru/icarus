use dotenvy::dotenv;

use tracing::info;

mod agents;
mod api;
mod live;
mod schema;
mod state;
mod tasks;
mod utils;

use utils::*;

#[tokio::main]
async fn main() {
    dotenv().ok();

    start_logger();

    info!("Starting Icarus Servers...\n");
    let state = state::GlobalState::new();

    let (api_server_handle, api_addr) = start_api_server(state.clone());
    info!("API Server listening at : {}", api_addr);

    let (rt_server_handle, rt_addr) = start_rt_server(state.clone());
    info!("RT  Server listening at : {}", rt_addr);

    match tokio::try_join!(api_server_handle, rt_server_handle) {
        Ok(_) => (),
        Err(e) => info!("Error starting servers: {:?}", e),
    };
}
