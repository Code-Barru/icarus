use reqwest::{multipart, Body};
use std::sync::Arc;
use tokio::{fs::File, sync::Mutex};
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::State;
use shared::models::Task;

pub async fn execute(
    input: &str,
    state: &Arc<Mutex<State>>,
    task: &Task,
) -> Result<Box<str>, Box<dyn std::error::Error + Send + Sync>> {
    let file_path = input;

    let file = match File::open(&file_path).await {
        Ok(file) => file,
        Err(_) => {
            return Err("Failed to open file".into());
        }
    };

    let stream = FramedRead::new(file, BytesCodec::new());
    let stream = Body::wrap_stream(stream);

    let file = multipart::Part::stream(stream);

    let form = multipart::Form::new().part("file", file);

    let state = state.lock().await;

    let request = reqwest::Client::new()
        .post(&format!(
            "{}/explorer/{}/upload",
            state.remote_server, task.uuid
        ))
        .multipart(form)
        .send()
        .await;

    match request {
        Ok(_) => (),
        Err(_) => return Err("Failed to upload file".into()),
    }

    Ok(Box::from("Ok"))
}
