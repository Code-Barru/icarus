use crate::AppState;

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use http::StatusCode;
use shared::models::{Directory, Task};

#[derive(serde::Deserialize, Debug, Clone)]
struct QueryDirectory {
    path: String,
    force: Option<bool>,
}

#[axum::debug_handler]
async fn get_directory(
    state: State<AppState>,
    Path(id): Path<uuid::Uuid>,
    path: Option<Query<QueryDirectory>>,
) -> impl IntoResponse {
    let directories = match state.directories.lock() {
        Ok(directories) => directories,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Failed to lock directories"),
            )
                .into_response()
        }
    };
    let path_clone = path.clone();
    let directories: Vec<_> = if let Some(path) = path {
        directories
            .iter()
            .filter(|d| d.agent == id && d.path == path.path)
            .cloned()
            .collect()
    } else {
        directories
            .iter()
            .filter(|d| d.agent == id)
            .cloned()
            .collect()
    };

    let force = match path_clone.clone() {
        Some(path) => path.force.unwrap_or(false),
        None => false,
    };

    if !directories.is_empty() && !force {
        return Json(directories).into_response();
    }

    let agents = match state.agents.lock() {
        Ok(agents) => agents,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Failed to lock agents"),
            )
                .into_response()
        }
    };

    let agent = match agents.iter().find(|a| a.uuid == id) {
        Some(agent) => agent,
        None => {
            return (StatusCode::NOT_FOUND, Json("Agent not found")).into_response();
        }
    };

    let directory_task = Task {
        uuid: uuid::Uuid::new_v4(),
        agent: agent.uuid,
        input: Some(path_clone.unwrap().path.clone()),
        response: None,
        status: shared::models::TaskStatus::Pending,
        task_type: shared::models::TaskType::Explorer,
        emitted_at: chrono::Utc::now().timestamp(),
        completed_at: 0,
    };

    let mut tasks = match state.tasks.lock() {
        Ok(tasks) => tasks,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Failed to lock tasks"),
            )
                .into_response()
        }
    };

    if !cfg!(not(debug_assertions)) {
        match state.io.emit("task_create", directory_task.clone()) {
            Ok(_) => (),
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json("Failed to emit task"),
                )
                    .into_response()
            }
        };
    }

    tasks.push(directory_task);

    return (StatusCode::CREATED, Json("Task created")).into_response();
}

async fn get_all_directories(state: State<AppState>) -> impl IntoResponse {
    let directories = match state.directories.lock() {
        Ok(directories) => directories,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Failed to lock directories"),
            )
                .into_response()
        }
    };

    Json(directories.clone()).into_response()
}

async fn create_directory(
    state: State<AppState>,
    Path(id): Path<uuid::Uuid>,
    Json(payload): Json<Directory>,
) -> impl IntoResponse {
    let agents = match state.agents.lock() {
        Ok(agents) => agents,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Failed to lock agents"),
            )
                .into_response()
        }
    };

    let agent = match agents.iter().find(|a| a.uuid == id) {
        Some(agent) => agent,
        None => {
            return (StatusCode::NOT_FOUND, Json("Agent not found")).into_response();
        }
    };

    let directory = Directory {
        agent: agent.uuid,
        ..payload.clone()
    };

    let mut directories = match state.directories.lock() {
        Ok(directories) => directories,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Failed to lock directories"),
            )
                .into_response()
        }
    };

    if let Some(existing_directory) = directories
        .iter_mut()
        .find(|d| d.agent == agent.uuid && d.path == payload.path)
    {
        *existing_directory = directory.clone(); // replace existing directory with new one
        match state.io.emit("directory_update", directory.clone()) {
            Ok(_) => (),
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json("Failed to emit directory"),
                )
                    .into_response()
            }
        };
        return (StatusCode::OK, Json("Directory updated")).into_response();
    }

    directories.push(directory.clone());
    match state.io.emit("directory_create", directory.clone()) {
        Ok(_) => (),
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Failed to emit directory"),
            )
                .into_response()
        }
    };

    (StatusCode::CREATED, Json("Directory created")).into_response()
}

pub fn get_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_all_directories))
        .with_state(state.clone())
        .route("/:id", get(get_directory))
        .with_state(state.clone())
        .route("/:id", post(create_directory))
        .with_state(state.clone())
}
