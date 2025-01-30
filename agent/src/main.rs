use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub mod utils;

#[tokio::main]
async fn main() {
    let state = utils::State::from("icarus".to_string());

    let mut client = match TcpStream::connect(format!("{}:{}", state.addr, state.rt_port)).await {
        Ok(client) => client,
        Err(e) => {
            return eprintln!("Error connecting to RT Server: {:?}", e);
        }
    };

    println!("Sending: {}", state.uuid);
    match client.write(state.uuid.as_bytes()).await {
        Ok(_) => (),
        Err(e) => {
            return eprintln!("Error writing to RT Server: {:?}", e);
        }
    }

    let mut buffer = [0; 1024];
    match client.read(&mut buffer).await {
        Ok(_) => (),
        Err(e) => {
            return eprintln!("Error reading from RT Server: {:?}", e);
        }
    }
}
