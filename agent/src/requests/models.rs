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
