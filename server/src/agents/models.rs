use core::fmt;

use crate::tasks::models::TaskEntry;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AgentEntry {
    pub uuid: Uuid,
    pub status: AgentStatus,

    #[serde(skip_serializing)]
    pub tasks: Vec<TaskEntry>,

    pub created_at: i64,
    pub last_seen_at: i64,
    pub last_seen_at_str: String,
    pub ip: String,
    pub hostname: String,
    pub platform: String,
}

impl fmt::Display for AgentEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Implement the formatting logic for AgentEntry here
        // For example, you can use `write!` macro to format the fields
        write!(f, "AgentEntry: {:?}", self)
    }
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

impl AgentStatus {
    #[allow(dead_code)]
    pub fn to_str(&self) -> &str {
        match self {
            AgentStatus::Online => "Online",
            AgentStatus::Offline => "Offline",
        }
    }
}

impl fmt::Display for AgentStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Implement the formatting logic for AgentStatus here
        // For example, you can use `write!` macro to format the enum variants
        write!(
            f,
            "{}",
            match self {
                AgentStatus::Online => "Online",
                AgentStatus::Offline => "Offline",
            }
        )
    }
}
