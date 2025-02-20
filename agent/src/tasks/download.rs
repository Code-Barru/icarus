// download means from agent to server

use std::{path::Path, thread};

use shared::{
    models::TaskStatus,
    packets::{DownloadRequest, Packet, TaskRequest, TaskResponse},
};
use tokio::{fs::File, io::AsyncReadExt};
// use tokio::{fs::File, io::AsyncReadExt};
use tracing::{debug, error, info};

use crate::rt_client::RTClient;

pub async fn execute(packet: &TaskRequest) -> TaskResponse {
    debug!("Executing download task");
    let path = match check_path(packet) {
        Ok(path) => path,
        Err(e) => {
            return TaskResponse::new(
                packet.task_uuid,
                TaskStatus::Failed,
                Some(e.to_string().as_bytes().to_vec()),
            );
        }
    };

    let client = match get_download_connection(packet).await {
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

    info!("Downloading file: {:?}", path.to_str().unwrap());

    let mut file = match File::open(&path).await {
        Ok(file) => file,
        Err(e) => {
            error!("Failed to open file: {:?}", e);
            return TaskResponse::new(
                packet.task_uuid,
                TaskStatus::Failed,
                Some(format!("Failed to open file: {}", e).as_bytes().to_vec()),
            );
        }
    };

    let mut buffer: [u8; 2048] = [0; 2048];

    loop {
        let bytes_read = match file.read(&mut buffer).await {
            Ok(bytes_read) => bytes_read,
            Err(e) => {
                error!("Failed to read from file: {:?}", e);
                return TaskResponse::new(
                    packet.task_uuid,
                    TaskStatus::Failed,
                    Some(
                        format!("Failed to read from file: {}", e)
                            .as_bytes()
                            .to_vec(),
                    ),
                );
            }
        };
        if bytes_read == 0 {
            break;
        }

        let data = &buffer[..bytes_read];
        client.send(data).await;
    }
    client.send(&[]).await;
    // let res = file.shutdown().await;
    client.disconnect().await;

    TaskResponse::new(
        packet.task_uuid,
        TaskStatus::Completed,
        Some("File downloaded".as_bytes().to_vec()),
    )
}

pub fn check_path(packet: &TaskRequest) -> Result<Box<Path>, Box<dyn std::error::Error>> {
    let parameter = match &packet.parameters {
        Some(parameters) => parameters.clone(),
        None => {
            return Err("No path provided".into());
        }
    };

    let path_str = match std::str::from_utf8(&parameter) {
        Ok(path) => path,
        Err(_) => {
            return Err("Invalid path provided".into());
        }
    };

    let path = std::path::Path::new(path_str);

    match path.try_exists() {
        Ok(exists) => {
            if !exists {
                return Err("File does not exist".into());
            };
            Ok(path.to_path_buf().into_boxed_path())
        }
        Err(e) => {
            return Err(format!("Error checking if file exists: {:?}", e).into());
        }
    }
}

pub async fn get_download_connection(
    packet: &TaskRequest,
) -> Result<RTClient, Box<dyn std::error::Error>> {
    let state = match crate::state::State::new("icarus") {
        Ok(state) => state,
        Err(e) => {
            error!("Failed to create state: {:?}", e);
            return Err(Box::new(e));
        }
    };

    let client = super::RTClient::new(
        state.addr,
        state.rt_port,
        shared::models::ConnectionType::FileDownload,
    )
    .await;
    match client.handshake(state.uuid).await {
        Ok(_) => (),
        Err(e) => {
            error!("Failed to handshake with download server: {:?}", e);
            return Err(e.into());
        }
    };

    // prevent from sending too fast
    thread::sleep(std::time::Duration::from_millis(5));

    let download_request = DownloadRequest::new(packet.task_uuid);
    client.send(&download_request.serialize()).await;

    let packet = match client.receive().await {
        Ok(packet) => packet,
        Err(e) => {
            error!("Failed to receive download response: {:?}", e);
            return Err(e.into());
        }
    };

    let download_response = match shared::packets::from_packet_bytes(&packet) {
        Ok(shared::packets::PacketEnum::DownloadResponse(response)) => response,
        Err(e) => {
            error!("Failed to deserialize download response: {:?}", e);
            return Err("Failed to deserialize download response".into());
        }
        _ => {
            error!("Invalid packet type");
            return Err("Invalid packet type".into());
        }
    };

    if !download_response.response {
        return Err("Task not found".into());
    }

    Ok(client)
}
