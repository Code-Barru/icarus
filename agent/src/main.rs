mod requests;
mod tasks;
use std::sync::{Arc, Mutex};

static TASKS_FETCH_INTERVAL: u64 = 3;
static REMOTE_SERVER: &str = "http://localhost:8080";

#[derive(Clone, Debug)]
pub struct State {
    uuid: uuid::Uuid,
    http: Arc<Mutex<reqwest::Client>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let http_client = reqwest::Client::new();
    let mut state = State {
        uuid: uuid::Uuid::new_v4(),
        http: Arc::new(Mutex::new(http_client)),
    };

    requests::agents::register(&mut state).await?;

    loop {
        requests::agents::get_tasks(&mut state).await?;
        tokio::time::sleep(std::time::Duration::from_secs(TASKS_FETCH_INTERVAL)).await;
    }
}
