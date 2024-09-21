mod requests;
mod tasks;
use std::sync::Arc;
use tokio::sync::Mutex;

static TASKS_FETCH_INTERVAL: u64 = 3;

#[derive(Clone, Debug)]
pub struct State {
    uuid: uuid::Uuid,
    http: Arc<Mutex<reqwest::Client>>,
    remote_server: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 && !cfg!(debug_assertions) {
        eprintln!("Usage: {} <remote_server>", args[0]);
        std::process::exit(1);
    }
    let remote_server = if cfg!(debug_assertions) {
        "http://localhost:1337".to_string()
    } else {
        args[1].clone()
    };

    let http_client = reqwest::Client::new();
    let mut state = State {
        uuid: uuid::Uuid::new_v4(),
        http: Arc::new(Mutex::new(http_client)),
        remote_server: remote_server.to_string(),
    };

    requests::agents::register(&mut state).await?;
    requests::agents::register_hardware(&mut state).await?;

    loop {
        requests::agents::get_tasks(&mut state).await?;
        tokio::time::sleep(std::time::Duration::from_secs(TASKS_FETCH_INTERVAL)).await;
    }
}
