use shared::packets::{DownloadResponse, Packet, PacketEnum, from_packet_bytes};
use tracing::error;

use super::Connection;
use tokio::{fs::File, io::AsyncWriteExt};

impl Connection {
    pub async fn handle_download_client(&mut self) {
        let mut file = match self.initialize_download().await {
            Ok(file) => file,
            Err(e) => {
                error!("Failed to initialize download: {:?}", e);
                return;
            }
        };

        loop {
            let data = match self.receive().await {
                Ok(data) => data,
                Err(e) => {
                    error!("Failed to receive data: {:?}", e);
                    return;
                }
            };
            if data.is_empty() {
                break;
            }

            match file.write_all(&data).await {
                Ok(_) => (),
                Err(e) => {
                    error!("Failed to write data to file: {:?}", e);
                    return;
                }
            };
        }

        match file.shutdown().await {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to shutdown file: {:?}", e);
                return;
            }
        }
    }

    async fn initialize_download(&mut self) -> Result<File, Box<dyn std::error::Error>> {
        let packet = match self.receive().await {
            Ok(data) => data,
            Err(e) => {
                error!("Failed to receive download request: {:?}", e);
                return Err(e.into());
            }
        };

        let download_request = match from_packet_bytes(&packet) {
            Ok(PacketEnum::DownloadRequest(request)) => request,
            Err(e) => {
                return Err(format!("Failed to deserialize download request: {:?}", e).into());
            }
            _ => {
                error!("Invalid packet type");
                return Err("Invalid packet type".into());
            }
        };

        let state = self.state.lock().await;
        let task = match state.get_task(download_request.task_uuid).await {
            Ok(task) => task,
            Err(e) => {
                error!("Failed to get task: {:?}", e);
                return Err(e.into());
            }
        };

        let download_response = match task {
            Some(_) => DownloadResponse::new(true),
            None => DownloadResponse::new(false),
        };

        if !download_response.response {
            return Err("Task not found".into());
        }

        let task_uuid = download_request.task_uuid.to_string();
        let file_path = format!("dist/download/{}", task_uuid);

        if let Err(e) = std::fs::create_dir_all("dist/download") {
            error!("Failed to create directory: {:?}", e);
            return Err(e.into());
        }

        let file = match File::create(&file_path).await {
            Ok(file) => file,
            Err(e) => {
                error!("Failed to create file: {:?}", e);
                return Err(e.into());
            }
        };

        self.send(&download_response.serialize()).await;

        Ok(file)
    }
}
