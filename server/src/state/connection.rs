use uuid::Uuid;

use crate::live::Connection;

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
}
