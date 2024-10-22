use axum::{http::StatusCode, response::IntoResponse};
use socketioxide::SocketIo;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

use shared::models::{Agent, AgentStatus};

pub async fn agents_health_check(agents: &mut Arc<Mutex<Vec<Agent>>>, io: &mut SocketIo) {
    let mut agents = agents.lock().await;
    if agents.is_empty() {
        return;
    }

    for agent in agents.iter_mut() {
        if agent.status == AgentStatus::Offline
            && agent.last_seen_at + (crate::AGENTS_HEALTH_CHECK_TIMEOUT as i64)
                > chrono::Utc::now().timestamp()
        {
            agent.status = AgentStatus::Online;
            match io.emit("agent_reconnect", agent.uuid) {
                Ok(_) => (),
                Err(_) => continue,
            };
            info!("Agent {} just came from the dead!", agent.uuid);
        }

        if agent.last_seen_at + (crate::AGENTS_HEALTH_CHECK_TIMEOUT as i64)
            < chrono::Utc::now().timestamp()
            && agent.status == AgentStatus::Online
        {
            agent.status = AgentStatus::Offline;
            match io.emit("agent_disconnect", agent.uuid) {
                Ok(_) => (),
                Err(_) => continue,
            };
            info!("Agent {} just went offline!", agent.uuid);
        }
    }
}

pub async fn not_found_handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404 Not Found")
}
