use std::{path::Path, sync::Arc};

use sha256::try_digest;
use shared::packets::{Packet, PacketEnum, TaskRequest, from_packet_bytes};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::Mutex,
};
use tracing::{debug, error, info};
use uuid::Uuid;

use super::packet_handler;
use crate::state::GlobalState;

use super::Connection;

impl Connection {
    pub fn new(
        socket: TcpStream,
        agent_uuid: Uuid,
        state: Arc<Mutex<GlobalState>>,
        shared_secret: [u8; 32],
    ) -> Self {
        let (read_socket, write_socket) = socket.into_split();
        Connection {
            agent_uuid,
            state,
            write_socket: Arc::new(Mutex::new(write_socket)),
            read_socket: Arc::new(Mutex::new(read_socket)),
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
        match self.send_update_request().await {
            Ok(need_update) => {
                if need_update {
                    return;
                }
            }
            Err(e) => {
                error!("Failed to send update request: {:?}", e);
                return;
            }
        };
        info!("Agent {:?} connected to RT Server", self.agent_uuid);
        self.send_undone_tasks().await;
        loop {
            let packet = match self.receive().await {
                Ok(data) => data,
                Err(_) => {
                    debug!("Error receiving data from Agent");
                    break;
                }
            };
            packet_handler::handle_packet(&packet, self.state.clone()).await;
        }
        info!("Agent {:?} disconnected from RT Server", self.agent_uuid);
    }

    pub async fn send_update_request(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let agent_file = Path::new("dist/agent");
        let agent_hash = match try_digest(agent_file) {
            Ok(hash) => hash,
            Err(e) => {
                return Err(e.into());
            }
        };
        let update_request = shared::packets::UpdateRequest::new(agent_hash);
        self.send(&update_request.serialize()).await;

        let packet = match self.receive().await {
            Ok(packet) => packet,
            Err(e) => {
                return Err(e.into());
            }
        };

        let update_response = match from_packet_bytes(&packet) {
            Ok(PacketEnum::UpdateResponse(update_response)) => update_response,
            _ => {
                return Err("Received unexpected packet type".into());
            }
        };

        if !update_response.need_update {
            return Ok(false);
        }

        info!("Agent {:?} needs to update", self.agent_uuid);

        match self.shutdown().await {
            Ok(_) => Ok(true),
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    async fn send_undone_tasks(&self) {
        let state = self.state.lock().await;
        let tasks = match state.get_undone_tasks(self.agent_uuid).await {
            Ok(tasks) => tasks,
            Err(e) => {
                error!("Failed to get undone tasks: {:?}", e);
                return;
            }
        };
        if tasks.is_empty() {
            return;
        }
        for task in tasks {
            let task_request = TaskRequest::new(task.id, task.task_type, task.parameters.clone());
            let data = task_request.serialize();
            self.send(&data).await;
        }
    }

    async fn shutdown(&self) -> Result<(), Box<dyn std::error::Error>> {
        let write_socket = self.write_socket.clone();
        let mut write_socket = write_socket.lock().await;

        match write_socket.shutdown().await {
            Ok(_) => (),
            Err(e) => {
                return Err(e.into());
            }
        };

        let state = self.state.lock().await;
        state.remove_connection(self.agent_uuid).await;
        Ok(())
    }
}
