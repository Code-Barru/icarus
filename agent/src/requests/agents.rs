use std::sync::Arc;
use tokio::sync::Mutex;

use crate::tasks::models::TaskEntry;
use crate::{requests::models::RegisterRequest, State};
use sysinfo::{Disks, Networks, System};

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

pub fn get_hardware() -> Result<super::models::AgentHardware, Box<dyn std::error::Error>> {
    let system = System::new_all();
    let disks = Disks::new_with_refreshed_list();
    let disks = disks
        .iter()
        .map(|disk| super::models::AgentDisk {
            total: disk.total_space(),
            free: disk.available_space(),
            used: disk.total_space() - disk.available_space(),
            name: disk.name().to_str().unwrap().to_string(),
            mount_point: disk.mount_point().to_str().unwrap().to_string(),
        })
        .collect();

    let network = Networks::new_with_refreshed_list();
    let mac = network
        .iter()
        .max_by_key(|(_, data)| data.total_packets_received())
        .map(|(_, data)| data.mac_address())
        .ok_or("No network interfaces found")?;

    Ok(super::models::AgentHardware {
        cpu: system.cpus()[0].brand().to_string().trim().to_string(),
        memory: format!("{}", system.total_memory()),
        disks,
        mac_address: mac.to_string(),
    })
}

pub async fn register_hardware(state: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    let hardware = match get_hardware() {
        Ok(hardware) => hardware,
        Err(e) => {
            eprintln!("Failed to get hardware: {}", e);
            return Ok(());
        }
    };

    let http = state.http.lock().await;
    let _ = http
        .post(format!(
            "{}/agents/{}/hardware",
            &state.remote_server, state.uuid
        ))
        .json(&hardware)
        .send()
        .await?;

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
