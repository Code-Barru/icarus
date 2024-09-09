use std::sync::{Arc, Mutex};

use axum::{http::StatusCode, response::IntoResponse};

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
        }

        if agent.last_seen_at + (crate::AGENTS_HEALTH_CHECK_TIMEOUT as i64)
            < chrono::Utc::now().timestamp()
            && agent.status == agents::models::AgentStatus::Online
        {
            agent.status = agents::models::AgentStatus::Offline;
            println!("Agent {} just went offline!", agent.uuid);
        }

        let seconds_since_last_seen = chrono::Utc::now().timestamp() - agent.last_seen_at;
        if seconds_since_last_seen < crate::AGENTS_HEALTH_CHECK_TIMEOUT as i64 {
            agent.last_seen_at_str = format!("Just checked in!");
            continue;
        }
        if seconds_since_last_seen < 60 {
            agent.last_seen_at_str =
                format!("Last checked in {} seconds ago.", seconds_since_last_seen);
            continue;
        } else if seconds_since_last_seen < 3600 {
            let minutes = seconds_since_last_seen / 60;
            agent.last_seen_at_str = format!(
                "Last checked in {} {} ago.",
                minutes,
                (if minutes == 1 { "minute" } else { "minutes" })
            );
            continue;
        } else if seconds_since_last_seen < 86400 {
            let hours = seconds_since_last_seen / 3600;
            agent.last_seen_at_str = format!(
                "Last checked in {} {} ago.",
                hours,
                if hours == 1 { "hour" } else { "hours" }
            );
            continue;
        }
    }
}

pub async fn get_favicon() -> Vec<u8> {
    include_bytes!("../templates/assets/favicon.ico").to_vec()
}

pub async fn not_found_handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404 Not Found")
}
