#![cfg_attr(
    all(target_os = "windows", not(debug_assertions),),
    windows_subsystem = "windows"
)]

pub mod rt_client;
pub mod state;
pub mod tasks;

use std::sync::Arc;

use rt_client::packet_handler::handle_packet;
use shared::models::ConnectionType;
use state::State;
use tokio::sync::Mutex;
use tracing::{error, info, level_filters::LevelFilter};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(LevelFilter::INFO)
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
        let rt_client =
            rt_client::RTClient::new(state.addr.clone(), state.rt_port, ConnectionType::Main).await;
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

        let rt_client = Arc::new(Mutex::new(rt_client));
        loop {
            let rt_client_clone = rt_client.clone();
            let rt_client = rt_client_clone.lock().await;

            let data = match rt_client.receive().await {
                Ok(data) => data,
                Err(e) => {
                    match e.kind() {
                        std::io::ErrorKind::UnexpectedEof => {
                            tokio::time::sleep(std::time::Duration::from_secs(60)).await;
                            break;
                        }
                        _ => (),
                    }
                    error!("Error receiving data from RT Server: {:?}", e);
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    break;
                }
            };
            let rt_client = rt_client.clone();
            tokio::spawn(async move {
                handle_packet(&data, rt_client.clone()).await;
            });
        }
    }
}
