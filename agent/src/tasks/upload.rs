use crate::State;
use shared::models::Task;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;

pub async fn execute(
    input: &str,
    state: &Arc<Mutex<State>>,
    task: &Task,
) -> Result<Box<str>, Box<dyn std::error::Error + Send + Sync>> {
    let state = state.lock().await;

    let request = reqwest::Client::new()
        .get(&format!(
            "{}/explorer/{}/download",
            state.remote_server, task.uuid
        ))
        .send()
        .await;

    let data = match request {
        Ok(response) => response.bytes().await,
        Err(_) => return Err("Failed to download file".into()),
    };

    let _ = match data {
        Ok(data) => {
            let file_path = input;
            if let Some(parent) = std::path::Path::new(file_path).parent() {
                tokio::fs::create_dir_all(parent).await?;
            }
            let mut file = match File::create(&file_path).await {
                Ok(file) => file,
                Err(err) => return Err(format!("Failed to create file:\n{}", err).into()),
            };

            match file.write(&data).await {
                Ok(_) => (),
                Err(_) => return Err("Failed to write file".into()),
            }
        }
        Err(_) => return Err("Failed to read file".into()),
    };

    Ok(Box::from("Ok"))
}
