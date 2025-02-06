use shared::packets::Packet;
use uuid::Uuid;

use crate::{live::Connection, tasks::models::Task};

use super::GlobalState;

impl GlobalState {
    pub async fn add_connection(&self, connection: Connection) {
        self.connections.lock().await.push(connection);
    }

    pub async fn remove_connection(&self, agent_uuid: Uuid) {
        let mut connections = self.connections.lock().await;
        connections.retain(|c| c.agent_uuid != agent_uuid);
    }

    pub async fn get_connection(&self, agent_uuid: Uuid) -> Option<Connection> {
        let connections = self.connections.lock().await;
        connections
            .iter()
            .find(|c| c.agent_uuid == agent_uuid)
            .map(|c| c.clone())
    }

    pub async fn send_task_request(
        &self,
        agent_uuid: Uuid,
        task: Task,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let connection = match self.get_connection(agent_uuid).await {
            Some(connection) => connection,
            None => return Ok(()),
        };
        connection.send(&task.to_packet().serialize()).await;

        Ok(())
    }
}
