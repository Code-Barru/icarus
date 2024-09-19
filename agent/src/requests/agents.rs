use std::sync::Arc;
use tokio::sync::Mutex;

use crate::tasks::models::TaskEntry;
use crate::{requests::models::RegisterRequest, State};
use sysinfo::System;

pub async fn register(state: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    let http = state.http.lock().await;

    let request = RegisterRequest {
        hostname: System::host_name().unwrap(),
        platform: format!(
            "{} {}",
            System::name().unwrap(),
            System::os_version().unwrap()
        ),
    };

    let request = http
        .post(format!("{}/agents/register", &state.remote_server))
        .json(&request)
        .send()
        .await?;
    let json = request.json::<super::models::RegisterResponse>().await?;

    state.uuid = json.uuid;
    Ok(())
}

pub async fn get_tasks(state: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    let http = state.http.lock().await;

    let request = http
        .get(format!(
            "{}/agents/{}/my_tasks",
            &state.remote_server, state.uuid
        ))
        .send()
        .await?;
    let json = request.json::<Vec<TaskEntry>>().await?;

    if json.len() == 0 {
        return Ok(());
    }

    for task in json.clone() {
        let state_clone = Arc::new(Mutex::new(state.clone()));
        tokio::spawn(async move {
            let _ = crate::tasks::task_handler(state_clone, task.clone()).await;
        });
    }

    Ok(())
}
