use shared::{
    models::{TaskStatus, TaskType},
    packets::{Packet, TaskRequest, TaskResponse},
};

use crate::rt_client::RTClient;

mod download;
mod shell_command;
mod upload;

pub async fn task_handler(rt_client: RTClient, packet: TaskRequest) {
    // Execution
    rt_client
        .send(&TaskResponse::new(packet.task_uuid, TaskStatus::Running, None).serialize())
        .await;

    let response = match packet.task_type {
        TaskType::ShellCommand => shell_command::execute(&packet).await,
        TaskType::FileUpload => upload::execute(&packet).await,
        TaskType::FileDownload => download::execute(&packet).await,
    };

    rt_client.send(&response.serialize()).await;
}
