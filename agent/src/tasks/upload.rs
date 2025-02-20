// upload means from server to agent

use shared::{
    models::TaskStatus,
    packets::{TaskRequest, TaskResponse},
};
use tracing::error;

use crate::rt_client::RTClient;

pub async fn execute(packet: &TaskRequest) -> TaskResponse {
    let path = match &packet.parameters {
        Some(parameters) => parameters.clone(),
        None => {
            return TaskResponse::new(
                packet.task_uuid,
                TaskStatus::Failed,
                Some("No path provided".as_bytes().to_vec()),
            );
        }
    };
    let _path = match std::str::from_utf8(&path) {
        Ok(path) => path,
        Err(_) => {
            return TaskResponse::new(
                packet.task_uuid,
                TaskStatus::Failed,
                Some("Invalid path provided".as_bytes().to_vec()),
            );
        }
    };

    let _client = match get_upload_connection().await {
        Ok(client) => client,
        Err(e) => {
            return TaskResponse::new(
                packet.task_uuid,
                TaskStatus::Failed,
                Some(
                    format!("Failed to connect to server: {}", e)
                        .as_bytes()
                        .to_vec(),
                ),
            );
        }
    };

    TaskResponse::new(
        packet.task_uuid,
        TaskStatus::Failed,
        Some("Not implemented".as_bytes().to_vec()),
    )
}

pub async fn get_upload_connection() -> Result<RTClient, Box<dyn std::error::Error>> {
    let state = match crate::state::State::new("icarus") {
        Ok(state) => state,
        Err(e) => {
            error!("Failed to create state: {:?}", e);
            return Err(e.into());
        }
    };

    let client = super::RTClient::new(
        state.addr,
        state.rt_port,
        shared::models::ConnectionType::FileUpload,
    )
    .await;
    match client.handshake(state.uuid).await {
        Ok(_) => (),
        Err(e) => {
            error!("Failed to handshake with update server: {:?}", e);
            return Err(e.into());
        }
    };
    Ok(client)
}
