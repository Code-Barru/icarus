use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
pub struct CreateAgent {
    pub hostname: String,
    pub platform: String,
}

#[derive(Deserialize, Clone)]
pub struct UpdateAgent {}
