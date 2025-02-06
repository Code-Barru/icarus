use sha256::try_digest;
use shared::packets::{Packet, UpdateRequest, UpdateResponse};
use tracing::{error, info};

pub async fn update_handler(rt_client: super::RTClient, update_packet: UpdateRequest) {
    let hash = match caclulate_hash() {
        Ok(hash) => hash,
        Err(e) => {
            error!("Failed to calculate hash: {:?}", e);
            return;
        }
    };

    if hash == update_packet.agent_hash {
        let update_response = UpdateResponse::new(false);
        rt_client.send(&update_response.serialize()).await;
        return;
    }

    let update_response = UpdateResponse::new(true);
    rt_client.send(&update_response.serialize()).await;
    info!("Need update, stopping agent");
    std::process::exit(0);
}

pub fn caclulate_hash() -> Result<String, Box<dyn std::error::Error>> {
    let file_path = match std::env::current_exe() {
        Ok(path) => path,
        Err(e) => {
            error!("Failed to get current executable path: {:?}", e);
            return Err(e.into());
        }
    };

    match try_digest(&file_path) {
        Ok(hash) => Ok(hash),
        Err(e) => {
            error!("Failed to calculate hash: {:?}", e);
            return Err(e.into());
        }
    }
}
