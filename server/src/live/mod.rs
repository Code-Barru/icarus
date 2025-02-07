#![allow(dead_code)]
use std::sync::Arc;

use tokio::{
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
    sync::Mutex,
};
use uuid::Uuid;

use crate::state::GlobalState;
use shared::models::ConnectionType;

mod connection;
mod handshake;
mod packet_handler;
mod rt_server;

pub struct RTServer {
    state: Arc<Mutex<GlobalState>>,
    rsa_key: Arc<Mutex<rsa::RsaPrivateKey>>,
}

#[derive(Clone)]
pub struct Connection {
    pub agent_uuid: Uuid,
    pub state: Arc<Mutex<GlobalState>>,
    pub write_socket: Arc<Mutex<OwnedWriteHalf>>,
    pub read_socket: Arc<Mutex<OwnedReadHalf>>,
    pub shared_secret: [u8; 32],
    pub connection_type: ConnectionType,
}
