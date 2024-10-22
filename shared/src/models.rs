use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Task {
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
    Explorer,
    FileDownload,
    FileUpload,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Agent {
    pub uuid: Uuid,
    pub status: AgentStatus,

    pub tasks: Vec<Uuid>,
    pub created_at: i64,
    pub last_seen_at: i64,
    pub ip: String,
    pub hostname: String,
    pub platform: String,
    pub hardware: Option<AgentHardware>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub enum AgentStatus {
    Online,
    Offline,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AgentHardware {
    pub cpu: String,
    pub memory: String,
    pub disks: Vec<AgentDisk>,
    pub mac_address: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AgentDisk {
    pub total: u64,
    pub free: u64,
    pub used: u64,
    pub name: String,
    pub mount_point: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Directory {
    pub agent: Uuid,
    pub path: String,
    pub files: Vec<File>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct File {
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
    pub created_at: i64,
    pub modified_at: i64,
}
