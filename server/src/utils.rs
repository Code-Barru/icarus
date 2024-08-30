use std::sync::{Arc, Mutex};

use crate::agents::{self, models::AgentEntry};

pub async fn agents_health_check(agents: &mut Arc<Mutex<Vec<AgentEntry>>>) {
    let mut agents = match agents.lock() {
        Ok(agents) => agents,
        Err(_) => return,
    };
    if agents.is_empty() {
        return;
    }

    for agent in agents.iter_mut() {
        if agent.status == agents::models::AgentStatus::Offline
            && agent.last_seen_at + (crate::AGENTS_HEALTH_CHECK_TIMEOUT as i64)
                > chrono::Utc::now().timestamp()
        {
            agent.status = agents::models::AgentStatus::Online;
            println!("Agent {} just came from the dead!", agent.uuid);
            continue;
        }

        if agent.status == agents::models::AgentStatus::Offline {
            continue;
        }

        if agent.last_seen_at + (crate::AGENTS_HEALTH_CHECK_TIMEOUT as i64)
            < chrono::Utc::now().timestamp()
        {
            agent.status = agents::models::AgentStatus::Offline;
            println!("Agent {} just went offline!", agent.uuid);
            continue;
        }
    }
}
