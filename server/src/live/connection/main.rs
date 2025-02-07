use std::path::Path;

use sha256::try_digest;
use shared::packets::{Packet, PacketEnum, TaskRequest, UpdateRequest, from_packet_bytes};
use tracing::{debug, error, info};

use super::super::packet_handler;

use super::Connection;
impl Connection {
    pub async fn handle_main_client(&mut self) {
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
        self.connect().await;
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
        let state = self.state.lock().await;
        match state.disconnect(self.agent_uuid).await {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to disconnect agent: {:?}", e);
            }
        };
    }

    pub async fn connect(&self) {
        let state = self.state.lock().await;
        match state.connect(self.agent_uuid).await {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to connect agent: {:?}", e);
            }
        };
    }

    pub async fn send_update_request(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let agent_file = Path::new("dist/agent");
        let agent_hash = match try_digest(agent_file) {
            Ok(hash) => hash,
            Err(e) => {
                return Err(e.into());
            }
        };
        let update_request = UpdateRequest::new(agent_hash);
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

        info!("Agent {:?} is updating..", self.agent_uuid);

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
        for task in tasks.clone() {
            let task_request = TaskRequest::new(task.id, task.task_type, task.parameters.clone());
            let data = task_request.serialize();
            self.send(&data).await;
        }
    }
}
