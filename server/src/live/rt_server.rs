use std::sync::Arc;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::Mutex,
};
use tracing::{debug, error};

use crate::state::GlobalState;

use super::RTServer;

impl RTServer {
    pub fn new(state: GlobalState) -> Self {
        let (priv_key, _) = shared::encryption::rsa::generate_keys();

        RTServer {
            state: Arc::new(Mutex::new(state)),
            rsa_key: Arc::new(Mutex::new(priv_key)),
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
                    debug!("Accepted connection from: {}", socket.peer_addr().unwrap());
                    (socket, socket_addr)
                }
                Err(e) => {
                    error!("Failed to accept socket: {}", e);
                    continue;
                }
            };
            let mut buffer = [0; 17];

            let _ = match self.receive(&mut socket).await {
                Ok(data) => buffer.copy_from_slice(&data),
                Err(_) => continue,
            };

            let mut connection = match self.handshake(buffer, socket).await {
                Ok(connection) => connection,
                Err(e) => {
                    error!("Failed to handshake with client {}: {:?}", socket_addr, e);
                    continue;
                }
            };

            let state = self.state.lock().await;
            state.add_connection(connection.clone()).await;

            let cloned_state = self.state.clone();
            tokio::spawn(async move {
                connection.handle_client().await;

                let state = cloned_state.lock().await;
                state.remove_connection(connection.agent_uuid).await;
            });
        }
    }

    pub async fn receive(
        &self,
        socket: &mut tokio::net::TcpStream,
    ) -> Result<Vec<u8>, std::io::Error> {
        let mut buf = [0; 1024];
        let n = match socket.read(&mut buf).await {
            Ok(n) => n,
            Err(e) => {
                error!("Failed to read data: {:?}", e);
                return Err(e);
            }
        };
        if n == 0 {
            error!("Client disconnected");
        }
        Ok(buf[..n].to_vec())
    }

    pub async fn send(
        &self,
        socket: &mut tokio::net::TcpStream,
        data: Vec<u8>,
    ) -> Result<(), std::io::Error> {
        match socket.write_all(&data).await {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to write data: {:?}", e);
                return Err(e);
            }
        };
        match socket.flush().await {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Failed to flush data: {:?}", e);
                return Err(e);
            }
        }
    }
}
