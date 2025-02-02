use std::{
    io::{Read, Write},
    sync::Arc,
};

use rsa::{
    RsaPrivateKey,
    pkcs8::{DecodePrivateKey, EncodePrivateKey},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::Mutex,
};
use tracing::{debug, error, info, warn};

use crate::state::GlobalState;

use super::RTServer;

impl RTServer {
    pub fn new(state: GlobalState) -> Self {
        let file_path = match std::env::var("RSA_PRIVATE_KEY_PATH") {
            Ok(path) => path,
            Err(_) => {
                warn!("RSA_PRIVATE_KEY_PATH not set, using default path");
                "private.pem".to_string()
            }
        };

        let priv_key = match RTServer::read_rsa_key(&file_path) {
            Ok(key) => key,
            Err(e) => {
                warn!("Failed to read RSA key: {}", e);
                info!("Generating new RSA key");
                let (priv_key, _) = shared::encryption::rsa::generate_keys();
                match RTServer::save_rsa_key(&file_path, priv_key.clone()) {
                    Ok(_) => info!("Saved RSA key to {}", file_path),
                    Err(e) => {
                        panic!("Failed to save RSA key: {}", e);
                    }
                }
                priv_key
            }
        };

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

            let _ = match receive(&mut socket).await {
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
    fn read_rsa_key(file_path: &str) -> Result<RsaPrivateKey, std::io::Error> {
        let mut file = match std::fs::File::open(file_path) {
            Ok(file) => file,
            Err(e) => {
                return Err(e);
            }
        };
        let mut buffer = Vec::new();
        match file.read_to_end(&mut buffer) {
            Ok(_) => (),
            Err(e) => {
                return Err(e);
            }
        };

        let content = match std::str::from_utf8(&buffer) {
            Ok(content) => content,
            Err(e) => {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e));
            }
        };

        match rsa::RsaPrivateKey::from_pkcs8_pem(&content) {
            Ok(key) => Ok(key),
            Err(e) => {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e));
            }
        }
    }

    fn save_rsa_key(file_path: &str, key: RsaPrivateKey) -> Result<(), std::io::Error> {
        let key = match key.to_pkcs8_pem(rsa::pkcs1::LineEnding::LF) {
            Ok(key) => key,
            Err(e) => {
                error!("Failed to convert key to PKCS1 PEM: {}", e);
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, e));
            }
        };
        let mut file = match std::fs::File::create(file_path) {
            Ok(file) => file,
            Err(e) => {
                error!("Failed to create file: {}", e);
                return Err(e);
            }
        };

        match file.write_all(key.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Failed to write to file: {}", e);
                return Err(e);
            }
        }
    }
}

pub async fn receive(socket: &mut tokio::net::TcpStream) -> Result<Vec<u8>, std::io::Error> {
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

pub async fn send(socket: &mut tokio::net::TcpStream, data: Vec<u8>) -> Result<(), std::io::Error> {
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
