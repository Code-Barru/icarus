use super::RTClient;
use std::{sync::Arc, vec};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::Mutex,
};
use tracing::error;

impl RTClient {
    pub async fn new(addr: String, port: u16) -> Self {
        let socket = loop {
            match TcpStream::connect(format!("{}:{}", addr, port)).await {
                Ok(stream) => break stream,
                Err(_) => {
                    error!("Error connecting to RT Server. Retrying in 5s");
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                }
            }
        };

        let (read_socket, write_socket) = socket.into_split();

        RTClient {
            write_socket: Arc::new(Mutex::new(write_socket)),
            read_socket: Arc::new(Mutex::new(read_socket)),
            shared_secret: Arc::new(Mutex::new([0; 32])),
        }
    }

    pub async fn receive_raw(&self) -> Result<Vec<u8>, std::io::Error> {
        let read_socket = self.read_socket.clone();
        let mut buf = [0; 4096];
        let mut read_socket = read_socket.lock().await;

        let n = match read_socket.read(&mut buf).await {
            Ok(n) => n,
            Err(e) => {
                error!("Failed to read data: {:?}", e);
                return Err(e);
            }
        };
        if n == 0 {}
        Ok(buf[..n].to_vec())
    }

    pub async fn send_raw(&self, data: Vec<u8>) {
        let write_socket = self.write_socket.clone();
        let mut write_socket = write_socket.lock().await;
        match write_socket.write_all(&data).await {
            Ok(_) => (),
            Err(e) => error!("Failed to send data: {:?}", e),
        };
        match write_socket.flush().await {
            Ok(_) => (),
            Err(e) => error!("Failed to flush data: {:?}", e),
        };
    }

    pub async fn receive(&self) -> Result<Vec<u8>, std::io::Error> {
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

        let shared_secret = self.shared_secret.lock().await;
        let data = shared::encryption::aes::decrypt(&shared_secret.clone(), &nonce, &ciphered_text);
        Ok(data)
    }

    pub async fn send(&self, data: &[u8]) {
        let shared_secret = self.shared_secret.lock().await;

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

    pub async fn disconnect(&self) {
        let write_socket = self.write_socket.clone();
        let mut write_socket = write_socket.lock().await;
        match write_socket.shutdown().await {
            Ok(_) => (),
            Err(e) => error!("Failed to shutdown socket: {:?}", e),
        };
    }
}
