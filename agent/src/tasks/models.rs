use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TaskEntry {
    pub uuid: Uuid,
    pub task_type: TaskType,
    pub agent: Uuid,
    pub status: TaskStatus,
    pub response: Option<String>,
    pub input: Option<String>,
    pub emitted_at: i64,
    pub completed_at: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TaskType {
    Shell,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

#[derive(Serialize, Clone, Debug)]
pub struct UpdateTask {
    pub status: TaskStatus,
    pub agent: Uuid,
    pub response: Option<String>,
}
