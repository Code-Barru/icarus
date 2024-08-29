use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone)]
pub struct TaskEntry {
    pub uuid: Uuid,
    pub task_type: TaskType,
    pub agent: Uuid,
    pub status: TaskStatus,
    pub response: String,
    pub emitted_at: i64,
    pub completed_at: i64,
}

#[derive(Deserialize, Clone)]
pub struct CreateTask {
    pub agent: Uuid,
    pub task_type: TaskType,
}

#[derive(Deserialize, Clone)]
pub struct UpdateTask {
    pub status: TaskStatus,
    pub agent: Uuid,
    pub response: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum TaskType {
    ShellCommand,
    PowerShellCommand,
}
