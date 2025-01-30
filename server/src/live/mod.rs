use std::sync::Arc;

use tokio::{net::TcpStream, sync::Mutex};
use uuid::Uuid;

use crate::state::GlobalState;

mod connection;
mod rt_server;

pub struct RTServer {
    state: Arc<Mutex<GlobalState>>,
}

pub struct Connection {
    pub socket: TcpStream,
    pub agent_uuid: Uuid,
}
