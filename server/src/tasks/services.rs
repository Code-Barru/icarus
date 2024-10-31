use super::models::{CreateTask, UpdateTask};
use crate::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use serde_json::json;
use shared::models::{Task, TaskStatus, TaskType};
use uuid::Uuid;

async fn get_tasks(state: State<AppState>) -> impl IntoResponse {
    let tasks = state.tasks.lock().await;
    let tasks: Vec<Task> = tasks
        .clone()
        .into_iter()
        .filter(|task| task.task_type != TaskType::Explorer)
        .collect();

    Json(tasks).into_response()
}

async fn get_single_task(state: State<AppState>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let tasks = state.tasks.lock().await;
    let task: Option<&Task> = tasks.iter().find(|task| task.uuid == id);
    match task {
        Some(task) => Json(task).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

async fn create_tasks(
    state: State<AppState>,
    Json(payload): Json<CreateTask>,
) -> impl IntoResponse {
    let mut agents = state.agents.lock().await;
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
    let task = Task {
        uuid: Uuid::new_v4(),
        status: TaskStatus::Pending,
        emitted_at: chrono::Utc::now().timestamp(),
        task_type: payload.task_type,
        agent: agent.uuid,
        response: Some("N/A".to_string()),
        input: if Some(payload.input.clone()) == None {
            None
        } else {
            match payload.input.clone() {
                Some(input) => Some(input),
                None => None,
            }
        },
        completed_at: 0,
    };
    if task.task_type == TaskType::Explorer && cfg!(not(debug_assertions)) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Explorer tasks are not allowed"})),
        )
            .into_response();
    }
    match state.io.emit("task_create", task.clone()) {
        Ok(_) => (),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    agent.tasks.push(task.uuid);

    let mut tasks = state.tasks.lock().await;
    tasks.push(task.clone());

    (StatusCode::CREATED, Json(task)).into_response()
}

async fn update_tasks(
    Path(id): Path<Uuid>,
    state: State<AppState>,
    Json(payload): Json<UpdateTask>,
) -> impl IntoResponse {
    let mut tasks = state.tasks.lock().await;

    let task = match tasks.iter_mut().find(|task| task.uuid == id) {
        Some(task) => task,
        None => return StatusCode::NOT_FOUND.into_response(),
    };

    task.status = payload.status;

    task.response = if Some(payload.response.clone()) == None {
        None
    } else {
        match payload.response {
            Some(response) => Some(response),
            None => None,
        }
    };
    task.completed_at = chrono::Utc::now().timestamp();
    match state.io.emit("task_update", task.clone()) {
        Ok(_) => (),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    Json(task.clone()).into_response()
}

pub fn get_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_tasks))
        .with_state(state.clone())
        .route("/:id", get(get_single_task))
        .with_state(state.clone())
        .route("/", post(create_tasks))
        .with_state(state.clone())
        .route("/:id", put(update_tasks))
        .with_state(state.clone())
}
