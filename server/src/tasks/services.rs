use super::models::{CreateTask, TaskEntry, TaskStatus, UpdateTask};
use crate::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use serde_json::json;
use uuid::Uuid;

async fn get_tasks(state: State<AppState>) -> impl IntoResponse {
    let agents = state.agents.lock().unwrap();
    let tasks: Vec<TaskEntry> = agents
        .iter()
        .flat_map(|agent| agent.tasks.clone())
        .collect();
    Json(tasks).into_response()
}

async fn create_tasks(
    state: State<AppState>,
    Json(payload): Json<CreateTask>,
) -> impl IntoResponse {
    let mut agents = match state.agents.lock() {
        Ok(agents) => agents,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    let agent = match agents.iter_mut().find(|agent| agent.uuid == payload.agent) {
        Some(agent) => agent,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Agent not found"})),
            )
                .into_response()
        }
    };
    let task = TaskEntry {
        uuid: Uuid::new_v4(),
        status: TaskStatus::Pending,
        emitted_at: chrono::Utc::now().timestamp(),
        task_type: payload.task_type,
        agent: agent.uuid,
        response: "".to_string(),
        completed_at: 0,
    };
    agent.tasks.push(task.clone());
    (StatusCode::CREATED, Json(task)).into_response()
}

async fn update_tasks(
    Path(id): Path<Uuid>,
    state: State<AppState>,
    Json(payload): Json<UpdateTask>,
) -> impl IntoResponse {
    let mut agents = match state.agents.lock() {
        Ok(agents) => agents,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    let agent = match agents.iter_mut().find(|agent| agent.uuid == payload.agent) {
        Some(agent) => agent,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Agent not found"})),
            )
                .into_response()
        }
    };

    let task = agent.tasks.iter_mut().find(|task| task.uuid == id).unwrap();
    task.status = payload.status;
    task.response = payload.response.clone();
    task.completed_at = chrono::Utc::now().timestamp();
    Json(task.clone()).into_response()
}

pub fn get_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_tasks))
        .with_state(state.clone())
        .route("/", post(create_tasks))
        .with_state(state.clone())
        .route("/:id", put(update_tasks))
        .with_state(state.clone())
}
