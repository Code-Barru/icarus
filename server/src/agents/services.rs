use std::net::SocketAddr;

use super::models::{AgentEntry, AgentHardware, AgentStatus, CreateAgent};
use crate::{
    tasks::models::{TaskEntry, TaskStatus},
    AppState,
};

use axum::{
    extract::{ConnectInfo, Path, State},
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

async fn get_single_agent(Path(id): Path<Uuid>, state: State<AppState>) -> impl IntoResponse {
    let agents = match state.agents.lock() {
        Ok(agents) => agents,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    let agent = agents.iter().find(|agent| agent.uuid == id);
    match agent {
        Some(agent) => Json(agent.clone()).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn get_my_tasks(state: State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let mut agents = match state.agents.lock() {
        Ok(agents) => agents,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    let agent = match agents.iter_mut().find(|agent| agent.uuid == id) {
        Some(agent) => agent,
        None => return StatusCode::NOT_FOUND.into_response(),
    };
    agent.last_seen_at = chrono::Utc::now().timestamp();

    let mut tasks = match state.tasks.lock() {
        Ok(tasks) => tasks,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    let mut tasks = tasks
        .iter_mut()
        .filter(|task| task.agent == id && task.status == TaskStatus::Pending);
    let mut tasks_reponse = vec![];

    for task in &mut tasks {
        task.status = TaskStatus::InProgress;

        match state.io.emit("task_update", task.clone()) {
            Ok(_) => (),
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        tasks_reponse.push(task.clone());
    }
    (StatusCode::OK, Json(tasks_reponse)).into_response()
}

async fn create_agent_hardware(
    state: State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<AgentHardware>,
) -> impl IntoResponse {
    let mut agents = match state.agents.lock() {
        Ok(agents) => agents,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    let agent = agents.iter_mut().find(|agent| agent.uuid == id);
    match agent {
        Some(agent) => {
            agent.hardware = Some(payload.clone());
            match state.io.emit("agent_create", agent.clone()) {
                Ok(_) => (),
                Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            };
            (StatusCode::OK, Json(json!({"message": "Hardware updated"}))).into_response()
        }
        None => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Agent not found"})),
        )
            .into_response(),
    }
}

async fn create_agents(
    state: State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
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
        ip: addr.ip().to_string(),
        hardware: None,
        hostname: payload.hostname,
        platform: payload.platform,
    };
    agents.push(agent.clone());
    (StatusCode::CREATED, Json(json!({"uuid": agent.uuid}))).into_response()
}

async fn delete_agents(Path(id): Path<Uuid>, state: State<AppState>) -> impl IntoResponse {
    let mut agents = match state.agents.lock() {
        Ok(agents) => agents,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    let index = agents.iter().position(|agent| agent.uuid == id);
    match state.io.emit("agent_delete", id) {
        Ok(_) => (),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
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

async fn get_tasks(Path(id): Path<Uuid>, state: State<AppState>) -> impl IntoResponse {
    let agents = match state.agents.lock() {
        Ok(agents) => agents,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    let agent = agents.iter().find(|agent| agent.uuid == id);
    match agent {
        Some(agent) => agent,
        None => return StatusCode::NOT_FOUND.into_response(),
    };

    let tasks = match state.tasks.lock() {
        Ok(tasks) => tasks,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    let tasks = tasks
        .iter()
        .filter(|task| task.agent == id)
        .cloned()
        .collect::<Vec<TaskEntry>>();

    (StatusCode::OK, Json(tasks)).into_response()
}

pub fn get_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_agents))
        .with_state(state.clone())
        .route("/:id", get(get_single_agent))
        .with_state(state.clone())
        .route("/register", post(create_agents))
        .with_state(state.clone())
        .route("/:id", delete(delete_agents))
        .with_state(state.clone())
        .route("/:id/my_tasks", get(get_my_tasks))
        .with_state(state.clone())
        .route("/:id/tasks", get(get_tasks))
        .with_state(state.clone())
        .route("/:id/hardware", post(create_agent_hardware))
        .with_state(state.clone())
}
