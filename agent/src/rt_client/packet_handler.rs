use shared::packets::{PacketEnum, from_packet_bytes};
use tracing::error;

use super::update::update_handler;
use crate::tasks::task_handler;

pub async fn handle_packet(packet: &[u8], rt_client: super::RTClient) {
    match from_packet_bytes(packet) {
        Ok(PacketEnum::TaskRequest(task_packet)) => task_handler(rt_client, task_packet).await,
        Ok(PacketEnum::UpdateRequest(update_packet)) => {
            update_handler(rt_client, update_packet).await
        }
        Err(e) => {
            error!("Failed to parse task request packet: {:?}", e);
            return;
        }
        _ => {
            error!("Received unexpected packet type");
            return;
        }
    };
}
