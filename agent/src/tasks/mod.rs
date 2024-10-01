use std::{error::Error, sync::Arc};
use tokio::sync::Mutex;

use models::TaskEntry;

use crate::State;

pub mod models;
mod shell;

pub async fn task_handler(
    state: Arc<Mutex<State>>,
    task: TaskEntry,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let task_clone = task.clone();
    let output = match task_clone.task_type {
        models::TaskType::Shell => {
            let input = match task_clone.input {
                Some(input) => input,
                None => {
                    let err: Box<dyn Error + Send + Sync> = Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Failed to get input for shell command.",
                    ));
                    return Err(err);
                }
            };
            shell::execute(&input)
        }
    };

    send_response(output, &state, task).await;
    Ok(())
}

pub async fn send_response(
    output: Result<Box<str>, Box<dyn std::error::Error + Send + Sync>>,
    state: &Arc<Mutex<State>>,
    task: TaskEntry,
) {
    let state = state.lock().await;
    let http = state.http.lock().await;

    let response = match output {
        Ok(output) => output,
        Err(e) => {
            let update = models::UpdateTask {
                status: models::TaskStatus::Failed,
                agent: task.agent,
                response: Some(e.to_string()),
            };
            let _ = http
                .put(format!("{}/tasks/{}", state.remote_server, task.uuid))
                .json(&update)
                .send()
                .await;
            return;
        }
    };

    let update = models::UpdateTask {
        status: models::TaskStatus::Completed,
        agent: task.agent,
        response: Some(response.to_string()),
    };
    let _ = http
        .put(format!("{}/tasks/{}", state.remote_server, task.uuid))
        .json(&update)
        .send()
        .await;
}
