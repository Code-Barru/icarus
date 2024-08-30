use std::sync::Arc;
use tokio::sync::Mutex;

use models::TaskEntry;

use crate::State;

pub mod models;

pub async fn task_handler(
    state: Arc<Mutex<State>>,
    task: TaskEntry,
) -> Result<(), Box<dyn std::error::Error>> {
    let state = state.lock().await;
    let http = match state.http.lock() {
        Ok(http) => http.clone(),
        Err(_) => return Err("Failed to lock http client".into()),
    };

    let update_task = models::UpdateTask {
        status: models::TaskStatus::Completed,
        agent: state.uuid,
        response: Some("Task completed successfully".to_string()),
    };

    let res = http
        .put(format!("{}/tasks/{}", crate::REMOTE_SERVER, task.uuid))
        .json(&update_task)
        .send()
        .await;

    Ok(())
}
