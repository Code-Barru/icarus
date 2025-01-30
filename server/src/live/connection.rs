use tokio::net::TcpStream;
use uuid::Uuid;

use super::Connection;

impl Connection {
    pub fn _new(socket: TcpStream, agent_uuid: Uuid) -> Self {
        Connection { socket, agent_uuid }
    }

    pub async fn _handle_client(&mut self) {
        let mut _buffer = [0; 1024];
    }
}
