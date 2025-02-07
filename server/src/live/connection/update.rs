use tokio::{fs::File, io::AsyncReadExt};
use tracing::error;

use super::Connection;

impl Connection {
    pub async fn handle_update_client(&self) {
        let mut update_file = match File::open("dist/agent").await {
            Ok(file) => file,
            Err(e) => {
                error!("Failed to open update file: {:?}", e);
                return;
            }
        };
        let mut buffer: [u8; 2048] = [0; 2048];

        loop {
            let bytes_read = match update_file.read(&mut buffer).await {
                Ok(bytes_read) => bytes_read,
                Err(e) => {
                    error!("Failed to read from file: {:?}", e);
                    return;
                }
            };
            if bytes_read == 0 {
                break;
            }

            let data = &buffer[..bytes_read];
            self.send(data).await;
        }
        self.send(&[]).await;
    }
}
