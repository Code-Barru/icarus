use crate::tasks::models::TaskEntry;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AgentEntry {
    pub uuid: Uuid,
    pub status: AgentStatus,

    pub tasks: Vec<TaskEntry>,
    pub created_at: i64,
    pub last_seen_at: i64,
    pub ip: String,
    pub hostname: String,
    pub platform: String,
    pub hardware: Option<AgentHardware>,
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

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
pub struct CreateAgent {
    pub hostname: String,
    pub platform: String,
}

#[derive(Deserialize, Clone)]
pub struct UpdateAgent {}

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub enum AgentStatus {
    Online,
    Offline,
}
