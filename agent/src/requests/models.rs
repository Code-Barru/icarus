use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct RegisterResponse {
    pub uuid: Uuid,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RegisterRequest {
    pub hostname: String,
    pub platform: String,
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
