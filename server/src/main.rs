use std::sync::{Arc, Mutex};

use axum::Router;
mod agents;
use agents::models::AgentEntry;
use tokio::net::TcpListener;

mod tasks;

static AGENTS_HEALTH_CHECK_INTERVAL: u64 = 5;
static AGENTS_HEALTH_CHECK_TIMEOUT: u64 = 3 * AGENTS_HEALTH_CHECK_INTERVAL;

#[derive(Clone)]
struct AppState {
    agents: Arc<Mutex<Vec<AgentEntry>>>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        agents: Arc::new(Mutex::new(Vec::new())),
    };

    let app = Router::new()
        .nest("/agents", agents::services::get_router(state.clone()))
        .nest("/tasks", tasks::services::get_router(state.clone()));

    let listener = match TcpListener::bind("0.0.0.0:8080").await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind to port 8080: {}", e);
            return;
        }
    };

    tokio::spawn(async move {
        let mut agents_clone = state.agents.clone();
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(AGENTS_HEALTH_CHECK_INTERVAL)).await;
            agents_health_check(&mut agents_clone).await;
        }
    });

    match axum::serve(listener, app).await {
        Ok(_) => println!("Server exited successfully"),
        Err(e) => eprintln!("Server exited with an error: {}", e),
    };
}

async fn agents_health_check(agents: &mut Arc<Mutex<Vec<AgentEntry>>>) {
    let mut agents = match agents.lock() {
        Ok(agents) => agents,
        Err(_) => return,
    };
    if agents.is_empty() {
        return;
    }

    for agent in agents.iter_mut() {
        if agent.status == agents::models::AgentStatus::Offline
            && agent.last_seen_at + (AGENTS_HEALTH_CHECK_TIMEOUT as i64)
                > chrono::Utc::now().timestamp()
        {
            agent.status = agents::models::AgentStatus::Online;
            println!("Agent {} just came from the dead!", agent.uuid);
            continue;
        }

        if agent.status == agents::models::AgentStatus::Offline {
            continue;
        }

        if agent.last_seen_at + (AGENTS_HEALTH_CHECK_TIMEOUT as i64)
            < chrono::Utc::now().timestamp()
        {
            agent.status = agents::models::AgentStatus::Offline;
            println!("Agent {} just went offline!", agent.uuid);
            continue;
        }
    }
}
