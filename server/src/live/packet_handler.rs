use crate::state::GlobalState;
use shared::packets::{PacketEnum, from_packet_bytes};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::error;

pub async fn handle_packet(packet: &[u8], state: Arc<Mutex<GlobalState>>) {
    match from_packet_bytes(packet) {
        Ok(PacketEnum::TaskResponse(task_packet)) => {
            let state = state.lock().await;
            match state.update_task(task_packet).await {
                Ok(_) => (),
                Err(e) => {
                    error!("Failed to update task: {:?}", e);
                }
            }
        }
        Err(e) => {
            error!("Failed to parse packet: {:?}", e);
        }
        _ => {
            error!("Received unexpected packet type");
        }
    }
}
