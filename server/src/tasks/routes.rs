use super::models::CreateTask;
use axum::{
    Json, Router,
    extract::State,
    response::IntoResponse,
    routing::{get, post},
};
use http::StatusCode;
use tracing::error;

use super::models::Task;
use crate::state::GlobalState;

pub async fn get_tasks(State(state): State<GlobalState>) -> impl IntoResponse {
    let tasks = match state.get_tasks().await {
        Ok(tasks) => tasks,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Error getting tasks"),
            )
                .into_response();
        }
    };

    (StatusCode::OK, Json(tasks)).into_response()
}

pub async fn create_task(
    State(state): State<GlobalState>,
    Json(create_task): Json<CreateTask>,
) -> impl IntoResponse {
    let _ = match state.get_agent(create_task.agent_uuid).await {
        Ok(_) => (),
        Err(_) => {
            return (StatusCode::BAD_REQUEST, Json("Agent does not exist")).into_response();
        }
    };

    let task = Task::from(create_task);
    match state.send_task_request(task.agent_uuid, task.clone()).await {
        Ok(_) => (),
        Err(e) => {
            error!("Error sending task request: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Error sending task request"),
            )
                .into_response();
        }
    };
    match state.create_task(&task).await {
        Ok(_) => (),
        Err(e) => {
            error!("Error creating task: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Error creating task"),
            )
                .into_response();
        }
    };

    (StatusCode::CREATED, Json(task)).into_response()
}

pub fn get_router(state: &GlobalState) -> Router {
    Router::new()
        .route("/", get(get_tasks))
        .route("/", post(create_task))
        .with_state(state.clone())
}
