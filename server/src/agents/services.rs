use super::models::{AgentEntry, AgentStatus, CreateAgent};
use crate::AppState;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use serde_json::json;
use uuid::Uuid;

async fn get_agents(state: State<AppState>) -> impl IntoResponse {
    let agents = state.agents.lock().unwrap();
    Json(agents.clone()).into_response()
}

async fn get_my_tasks(state: State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let mut agents = match state.agents.lock() {
        Ok(agents) => agents,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    let agent = match agents.iter_mut().find(|agent| agent.uuid == id) {
        Some(agent) => {
            agent.last_seen_at = chrono::Utc::now().timestamp();
            agent.clone()
        }
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Agent not found"})),
            )
                .into_response()
        }
    };
    (StatusCode::OK, Json(agent.tasks.clone())).into_response()
}

async fn create_agents(
    state: State<AppState>,
    Json(payload): Json<CreateAgent>,
) -> impl IntoResponse {
    let mut agents = match state.agents.lock() {
        Ok(agents) => agents,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let agent = AgentEntry {
        uuid: Uuid::new_v4(),
        status: AgentStatus::Online,
        tasks: Vec::new(),
        created_at: chrono::Utc::now().timestamp(),
        last_seen_at: chrono::Utc::now().timestamp(),
        ip: "".to_string(),
        hostname: payload.hostname,
        platform: payload.platform,
    };
    agents.push(agent.clone());
    (StatusCode::CREATED, Json(agent)).into_response()
}

async fn delete_agents(Path(id): Path<Uuid>, state: State<AppState>) -> impl IntoResponse {
    let mut agents = match state.agents.lock() {
        Ok(agents) => agents,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    let index = agents.iter().position(|agent| agent.uuid == id);
    match index {
        Some(index) => {
            agents.remove(index);
            StatusCode::OK.into_response()
        }
        None => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Agent not found"})),
        )
            .into_response(),
    }
}

pub fn get_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_agents))
        .with_state(state.clone())
        .route("/register", post(create_agents))
        .with_state(state.clone())
        .route("/:id", delete(delete_agents))
        .with_state(state.clone())
        .route("/:id/my_tasks", get(get_my_tasks))
        .with_state(state.clone())
}
