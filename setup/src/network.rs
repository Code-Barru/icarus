use reqwest::Client;
use std::fs::File;
use std::io::Write;

pub async fn get_agent_file(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    let mut response = client.get(url).send().await?;

    if !response.status().is_success() {
        return Err("Failed to fetch file".into());
    }

    let temp = std::env::temp_dir();
    let temp_str = match temp.to_str() {
        Some(s) => s,
        None => {
            return Err("Failed to convert path to string".into());
        }
    };

    let file_path = format!("{}icarus", temp_str);
    let mut file = match File::create(&file_path) {
        Ok(f) => f,
        Err(e) => {
            return Err(e.into());
        }
    };

    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk)?;
    }

    Ok(file_path.to_string())
}

pub async fn register(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    // Make the POST request to register the agent
    let response = client.post(url).send().await?;

    // Check for success status code
    if !response.status().is_success() {
        return Err("Failed to register agent".into());
    }

    // Parse the response body as JSON
    let json: serde_json::Value = response.json().await?;

    // Extract the UUID field from the JSON response
    let uuid = json["uuid"]
        .as_str()
        .ok_or("UUID field is missing")?
        .to_string();

    Ok(uuid)
}
