use crate::tasks::models::TaskEntry;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone)]
pub struct AgentEntry {
    pub uuid: Uuid,
    pub status: AgentStatus,

    #[serde(skip_serializing)]
    pub tasks: Vec<TaskEntry>,

    pub created_at: i64,
    pub last_seen_at: i64,
    pub ip: String,
    pub hostname: String,
    pub platform: String,
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
#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub enum AgentStatus {
    Online,
    Offline,
}
