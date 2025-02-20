use std::sync::Arc;

use shared::models::ConnectionType;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::Mutex,
};
use tracing::{debug, error};
use uuid::Uuid;

mod download;
mod main;
mod update;

use crate::state::GlobalState;

use super::Connection;

impl Connection {
    pub fn new(
        socket: TcpStream,
        agent_uuid: Uuid,
        state: Arc<Mutex<GlobalState>>,
        shared_secret: [u8; 32],
        connection_type: ConnectionType,
    ) -> Self {
        let (read_socket, write_socket) = socket.into_split();
        Connection {
            agent_uuid,
            state,
            write_socket: Arc::new(Mutex::new(write_socket)),
            read_socket: Arc::new(Mutex::new(read_socket)),
            shared_secret,
            connection_type,
        }
    }

    pub async fn send(&self, data: &[u8]) {
        let shared_secret = self.shared_secret.clone();

        let (ciphered_text, nonce) =
            shared::encryption::aes::encrypt(&shared_secret.clone(), &data);

        let mut data: Vec<u8> = Vec::new();
        let len: u32 = match ciphered_text.len().try_into() {
            Ok(len) => len,
            Err(e) => {
                error!("Failed to convert length: {:?}", e);
                return;
            }
        };

        data.extend_from_slice(&len.to_be_bytes());
        data.extend_from_slice(&nonce);
        data.extend_from_slice(&ciphered_text);

        let mut write_socket = self.write_socket.lock().await;
        match write_socket.write_all(&data).await {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to send data: {:?}", e);
            }
        };
        match write_socket.flush().await {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to flush data: {:?}", e);
            }
        };
    }

    pub async fn receive(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut read_socket = self.read_socket.lock().await;
        let mut buff_len: [u8; 4] = [0; 4];

        match read_socket.read_exact(&mut buff_len).await {
            Ok(_) => (),
            Err(e) => {
                debug!("Failed to read buff len: {:?}", e);
                return Err(e);
            }
        };

        let buff_len = u32::from_be_bytes(buff_len) as usize;
        let mut nonce: [u8; 12] = [0; 12];

        match read_socket.read_exact(&mut nonce).await {
            Ok(_) => (),
            Err(e) => {
                debug!("Failed to read nonce: {:?}", e);
                return Err(e);
            }
        };

        let mut ciphered_text: Vec<u8> = vec![0; buff_len];
        match read_socket.read(&mut ciphered_text).await {
            Ok(_) => (),
            Err(e) => {
                debug!("Failed to read ciphered text: {:?}", e);
                return Err(e);
            }
        };
        let data =
            shared::encryption::aes::decrypt(&self.shared_secret.clone(), &nonce, &ciphered_text);
        Ok(data)
    }

    pub async fn handle_client(&mut self) {
        match self.connection_type {
            ConnectionType::Main => self.handle_main_client().await,
            ConnectionType::Update => self.handle_update_client().await,
            ConnectionType::FileDownload => self.handle_download_client().await,
            _ => {
                error!("Unknown connection type");
            }
        }
    }

    pub async fn shutdown(&self) -> Result<(), Box<dyn std::error::Error>> {
        let write_socket = self.write_socket.clone();
        let mut write_socket = write_socket.lock().await;

        match write_socket.shutdown().await {
            Ok(_) => (),
            Err(e) => {
                return Err(e.into());
            }
        };

        let state = self.state.lock().await;
        state.remove_connection(self).await;
        Ok(())
    }
}
