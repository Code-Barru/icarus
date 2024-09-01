use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TaskEntry {
    pub uuid: Uuid,
    pub task_type: TaskType,
    pub agent: Uuid,
    pub agent_name: String,
    pub status: TaskStatus,
    pub response: Option<String>,
    pub input: Option<String>,
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
    pub agent: Uuid,
    pub response: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TaskStatus::Pending => write!(f, "Pending"),
            TaskStatus::InProgress => write!(f, "In Progress"),
            TaskStatus::Completed => write!(f, "Completed"),
            TaskStatus::Failed => write!(f, "Failed"),
        }
    }
}

impl TaskStatus {
    pub fn to_str(&self) -> &str {
        match self {
            TaskStatus::Pending => "Pending",
            TaskStatus::InProgress => "In Progress",
            TaskStatus::Completed => "Completed",
            TaskStatus::Failed => "Failed",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TaskType {
    ShellCommand,
    PowerShellCommand,
}
impl std::fmt::Display for TaskType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TaskType::ShellCommand => write!(f, "Shell Command"),
            TaskType::PowerShellCommand => write!(f, "PowerShell Command"),
        }
    }
}
