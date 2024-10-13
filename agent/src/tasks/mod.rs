use std::{error::Error, sync::Arc};
use tokio::sync::Mutex;

use shared::models::{Task, TaskStatus, TaskType};

use crate::State;

mod explorer;
pub mod models;
mod shell;

pub async fn task_handler(
    state: Arc<Mutex<State>>,
    task: Task,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let task_clone = task.clone();
    let output = match task_clone.task_type {
        TaskType::Shell => {
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
        TaskType::Explorer => {
            let input = match task_clone.input {
                Some(input) => input,
                None => {
                    let err: Box<dyn Error + Send + Sync> = Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Failed to get input for explorer command.",
                    ));
                    return Err(err);
                }
            };
            explorer::execute(&input, &state.clone()).await
        }
    };

    send_response(output, &state, task).await;
    Ok(())
}

pub async fn send_response(
    output: Result<Box<str>, Box<dyn std::error::Error + Send + Sync>>,
    state: &Arc<Mutex<State>>,
    task: Task,
) {
    let state = state.lock().await;
    let http = state.http.lock().await;

    let response = match output {
        Ok(output) => output,
        Err(e) => {
            let update = models::UpdateTask {
                status: TaskStatus::Failed,
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
        status: TaskStatus::Completed,
        agent: task.agent,
        response: Some(response.to_string()),
    };
    let _ = http
        .put(format!("{}/tasks/{}", state.remote_server, task.uuid))
        .json(&update)
        .send()
        .await;
}
