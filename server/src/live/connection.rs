use std::sync::Arc;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::Mutex,
};
use tracing::{error, info};
use uuid::Uuid;

use super::Connection;

impl Connection {
    pub fn new(socket: TcpStream, agent_uuid: Uuid, shared_secret: [u8; 32]) -> Self {
        let (read_socket, write_socket) = socket.into_split();
        Connection {
            write_socket: Arc::new(Mutex::new(write_socket)),
            read_socket: Arc::new(Mutex::new(read_socket)),
            agent_uuid,
            shared_secret: shared_secret,
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

    async fn receive(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut read_socket = self.read_socket.lock().await;
        let mut buff_len: [u8; 4] = [0; 4];

        match read_socket.read_exact(&mut buff_len).await {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to read data: {:?}", e);
                return Err(e);
            }
        };

        let buff_len = u32::from_be_bytes(buff_len) as usize;
        let mut nonce: [u8; 12] = [0; 12];

        match read_socket.read_exact(&mut nonce).await {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to read data: {:?}", e);
                return Err(e);
            }
        };

        let mut ciphered_text: Vec<u8> = vec![0; buff_len];
        match read_socket.read(&mut ciphered_text).await {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to read data: {:?}", e);
                return Err(e);
            }
        };
        let data =
            shared::encryption::aes::decrypt(&self.shared_secret.clone(), &nonce, &ciphered_text);
        Ok(data)
    }

    pub async fn handle_client(&mut self) {
        info!("Agent {:?} connected to RT Server", self.agent_uuid);
        loop {
            match self.receive().await {
                Ok(data) => {
                    info!("{}", String::from_utf8_lossy(&data));
                }
                Err(_) => {
                    error!("Error receiving data from RT Server. Reconnecting in 5s");
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    break;
                }
            };
        }
        info!("Agent {:?} disconnected from RT Server", self.agent_uuid);
    }
}
