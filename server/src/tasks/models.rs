use serde::Deserialize;
use shared::models::{TaskStatus, TaskType};
use uuid::Uuid;

#[derive(Deserialize, Clone)]
pub struct CreateTask {
    pub agent: Uuid,
    pub task_type: TaskType,
    pub input: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct UpdateTask {
    pub status: TaskStatus,
    pub response: Option<String>,
}
