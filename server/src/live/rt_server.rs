use std::sync::Arc;

use std::error::Error;
use tokio::{io::AsyncReadExt, sync::Mutex};
use tracing::{error, info};
use uuid::Uuid;

use crate::state::GlobalState;

use super::RTServer;

impl RTServer {
    pub fn new(state: GlobalState) -> Self {
        RTServer {
            state: Arc::new(Mutex::new(state)),
        }
    }

    pub async fn start_server(&self, addr: &std::net::SocketAddr) {
        let listener = match tokio::net::TcpListener::bind(addr).await {
            Ok(listener) => listener,
            Err(e) => {
                error!("Failed to bind: {}", e);
                return;
            }
        };

        loop {
            let (mut socket, socket_addr) = match listener.accept().await {
                Ok((socket, socket_addr)) => {
                    info!("Accepted connection from: {}", socket.peer_addr().unwrap());
                    (socket, socket_addr)
                }
                Err(e) => {
                    error!("Failed to accept socket: {}", e);
                    continue;
                }
            };
            let mut buffer = [0; 16];

            match socket.read(&mut buffer).await {
                Ok(0) => {
                    info!("Client {} disconnected.", socket_addr)
                }
                Ok(data) => {
                    match self.handshake(buffer, data).await {
                        Ok(uuid) => {
                            info!(
                                "Handshake successful with client {} - {}",
                                socket_addr, uuid
                            );
                        }
                        Err(e) => {
                            error!("Failed to handshake with client {}:{:?}", socket_addr, e)
                        }
                    };
                }
                Err(e) => {
                    error!("Failed to read from socket {}: {:?}", socket_addr, e)
                }
            }

            // tokio::spawn(async move {
            //     Self::handle_client(socket).await;
            // });
        }
    }
    pub async fn handshake(
        &self,
        data: [u8; 16],
        content_length: usize,
    ) -> Result<Uuid, Box<dyn Error>> {
        if content_length != 16 {
            return Err("Invalid UUID".into());
        }

        let uuid = Uuid::from_bytes(data);
        let state = self.state.lock().await;
        let _ = match state.get_agent(uuid).await {
            Ok(agent) => match agent {
                Some(_) => (),
                None => {
                    return Err(format!("Agent not found with uuid: {}", uuid).into());
                }
            },
            Err(e) => {
                return Err(e.into());
            }
        };
        Ok(uuid)
    }
}
