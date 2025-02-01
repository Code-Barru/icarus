#![windows_subsystem = "windows"]
pub mod rt_client;
pub mod state;
use state::State;
use tracing::{error, info, level_filters::LevelFilter};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(LevelFilter::DEBUG)
        .compact()
        .init();

    let state = match State::new("icarus") {
        Ok(state) => state,
        Err(_) => {
            error!("Failed to read state file. Retrying in 5s");
            std::process::exit(1);
        }
    };

    // Agent main loop
    // Agent keeps trying to reconnect if error occurs
    loop {
        let rt_client = rt_client::RTClient::new(state.addr.clone(), state.rt_port).await;
        info!("Connected to RT Server");
        match rt_client.handshake(state.uuid).await {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to handshake with RT Server: {:?}", e);
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                continue;
            }
        };
        info!("Handshake successful");

        loop {
            let data = match rt_client.receive().await {
                Ok(data) => data,
                Err(_) => {
                    error!("Error receiving data from RT Server. Reconnecting in 5s");
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    break;
                }
            };
            info!("{}", String::from_utf8_lossy(&data));
        }
    }
}
