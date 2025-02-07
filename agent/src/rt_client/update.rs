use sha256::try_digest;
use shared::packets::{Packet, UpdateRequest, UpdateResponse};
use tokio::{fs::File, io::AsyncWriteExt, process::Command};
use tracing::{error, info};

pub async fn update_handler(rt_client: super::RTClient, update_packet: UpdateRequest) {
    let hash = match caclulate_hash() {
        Ok(hash) => hash,
        Err(e) => {
            error!("Failed to calculate hash: {:?}", e);
            return;
        }
    };

    if hash == update_packet.agent_hash {
        let update_response = UpdateResponse::new(false);
        rt_client.send(&update_response.serialize()).await;
        return;
    }

    let update_response = UpdateResponse::new(true);
    rt_client.send(&update_response.serialize()).await;

    update().await;
    rt_client.disconnect().await;
    info!("Disconnected from RT Server");
    std::process::exit(0);
}

pub async fn update() {
    let state = match crate::state::State::new("icarus") {
        Ok(state) => state,
        Err(e) => {
            error!("Failed to create state: {:?}", e);
            return;
        }
    };

    let update_client = super::RTClient::new(
        state.addr,
        state.rt_port,
        shared::models::ConnectionType::Update,
    )
    .await;
    match update_client.handshake(state.uuid).await {
        Ok(_) => (),
        Err(e) => {
            error!("Failed to handshake with update server: {:?}", e);
            return;
        }
    };

    let current_exe_path = match std::env::current_exe() {
        Ok(path) => path,
        Err(e) => {
            error!("Failed to get current executable path: {:?}", e);
            return;
        }
    };

    let new_file_path = current_exe_path.with_extension(".new");

    let mut file = match File::create(&new_file_path).await {
        Ok(file) => file,
        Err(e) => {
            error!("Failed to create new executable file: {:?}", e);
            return;
        }
    };

    loop {
        let data = match update_client.receive().await {
            Ok(data) => data,
            Err(e) => {
                error!("Failed to receive data: {:?}", e);
                return;
            }
        };
        if data.is_empty() {
            break;
        }

        match file.write_all(&data).await {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to write data to file: {:?}", e);
                return;
            }
        };
    }
    update_client.disconnect().await;

    match file.shutdown().await {
        Ok(_) => (),
        Err(e) => {
            error!("Failed to shutdown file: {:?}", e);
            return;
        }
    };

    let bat_path = current_exe_path.with_file_name("rename_script.bat");

    let mut bat_file = match File::create(&bat_path).await {
        Ok(file) => file,
        Err(e) => {
            error!("Failed to create batch script: {:?}", e);
            return;
        }
    };

    let script_content = format!(
        "@echo off\n\
        timeout /t 2 /nobreak >nul\n\
        del \"{1}\"\n\
        move \"{0}\" \"{1}\"\n\
        start \"\" \"{1}\"\n\
        del \"%~f0\"\n",
        new_file_path.display(),
        current_exe_path.display(),
    );

    if let Err(e) = bat_file.write_all(script_content.as_bytes()).await {
        error!("Failed to write batch script: {:?}", e);
        return;
    }

    // Run the batch script and exit
    match Command::new("cmd")
        .args(["/C", bat_path.to_str().unwrap()])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
    {
        Ok(_) => (),
        Err(e) => {
            error!("Failed to run batch script: {:?}", e);
            return;
        }
    };

    std::process::exit(0);
}

pub fn caclulate_hash() -> Result<String, Box<dyn std::error::Error>> {
    let file_path = match std::env::current_exe() {
        Ok(path) => path,
        Err(e) => {
            error!("Failed to get current executable path: {:?}", e);
            return Err(e.into());
        }
    };

    match try_digest(&file_path) {
        Ok(hash) => Ok(hash),
        Err(e) => {
            error!("Failed to calculate hash: {:?}", e);
            return Err(e.into());
        }
    }
}
