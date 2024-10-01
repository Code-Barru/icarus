use serde::Serialize;
use shared::models::TaskStatus;
use uuid::Uuid;

#[derive(Serialize, Clone, Debug)]
pub struct UpdateTask {
    pub status: TaskStatus,
    pub agent: Uuid,
    pub response: Option<String>,
}
