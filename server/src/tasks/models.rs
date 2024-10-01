use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TaskEntry {
    pub uuid: Uuid,
    pub task_type: TaskType,
    pub agent: Uuid,
    pub agent_name: String,
    pub status: TaskStatus,
    pub response: String,
    pub input: String,
    pub emitted_at: i64,
    pub completed_at: i64,
}

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

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TaskType {
    Shell,
}
