use tokio::{fs::File, io::AsyncWriteExt};
use tokio_util::io::ReaderStream;

use crate::AppState;

use axum::{
    body::Body,
    extract::{Multipart, Path, Query, State},
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
    let directories = state.directories.lock().await;
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

    let agents = state.agents.lock().await;
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

    let mut tasks = state.tasks.lock().await;

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
    let directories = state.directories.lock().await;

    Json(directories.clone()).into_response()
}

async fn create_directory(
    state: State<AppState>,
    Path(id): Path<uuid::Uuid>,
    Json(payload): Json<Directory>,
) -> impl IntoResponse {
    let agents = state.agents.lock().await;

    let agent = match agents.iter().find(|a| a.uuid == id) {
        Some(agent) => agent,
        None => {
            return (StatusCode::NOT_FOUND, Json("Agent not found")).into_response();
        }
    };
    let directory = Directory {
        agent: agent.uuid,
        path: payload.path.clone().replace("\\", ""),
        files: payload.files.clone(),
    };

    let mut directories = state.directories.lock().await;
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

async fn upload(
    state: State<AppState>,
    Path(task_id): Path<uuid::Uuid>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let tasks = state.tasks.lock().await;
    let task = match tasks.iter().find(|t| t.uuid == task_id) {
        Some(task) => task.clone(),
        None => {
            return (StatusCode::NOT_FOUND, Json("Task not found")).into_response();
        }
    };

    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap();

        if field_name != "file" {
            continue;
        }

        let task_type: String;

        if task.task_type == shared::models::TaskType::FileUpload {
            task_type = "upload".to_string();
        } else if task.task_type == shared::models::TaskType::FileDownload {
            task_type = "download".to_string();
        } else {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json("Invalid task type")).into_response();
        }

        let file_path = format!("{}/{}", task_type, task.uuid);
        if File::open(&file_path).await.is_ok() {
            return (StatusCode::CONFLICT, Json("File already uploaded")).into_response();
        }

        let mut file = match File::create(&file_path).await {
            Ok(file) => file,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json("Failed to create file"),
                )
                    .into_response();
            }
        };
        let data = match field.bytes().await {
            Ok(data) => data,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json("Failed to read file"),
                )
                    .into_response();
            }
        };
        match file.write(&data).await {
            Ok(_) => (),

            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json("Failed to write file"),
                )
                    .into_response();
            }
        };
    }

    (StatusCode::OK).into_response()
}

async fn download(state: State<AppState>, Path(task_id): Path<uuid::Uuid>) -> impl IntoResponse {
    let tasks = state.tasks.lock().await;
    let task = match tasks.iter().find(|t| t.uuid == task_id) {
        Some(task) => task.clone(),
        None => {
            return (StatusCode::NOT_FOUND, Json("Task not found")).into_response();
        }
    };

    let task_type: String;

    if task.task_type == shared::models::TaskType::FileUpload {
        task_type = "upload".to_string();
    } else if task.task_type == shared::models::TaskType::FileDownload {
        task_type = "download".to_string();
    } else {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json("Invalid task type")).into_response();
    }

    let file_path = format!("{}/{}", task_type, task.uuid);

    let file = match File::open(&file_path).await {
        Ok(file) => file,
        Err(_) => {
            return (StatusCode::NOT_FOUND, Json("File not found")).into_response();
        }
    };

    let stream = ReaderStream::new(file);

    let body = Body::from_stream(stream);

    (body).into_response()
}

pub fn get_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(get_all_directories))
        .with_state(state.clone())
        .route("/:id", get(get_directory))
        .with_state(state.clone())
        .route("/:id", post(create_directory))
        .with_state(state.clone())
        .route("/:task_id/upload", post(upload))
        .with_state(state.clone())
        .route("/:task_id/download", get(download))
        .with_state(state.clone())
}
