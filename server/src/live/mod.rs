#![allow(dead_code)]
use std::sync::Arc;

use tokio::{
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
    sync::Mutex,
};
use uuid::Uuid;

use crate::state::GlobalState;

mod connection;
mod handshake;
mod rt_server;

pub struct RTServer {
    state: Arc<Mutex<GlobalState>>,
    rsa_key: Arc<Mutex<rsa::RsaPrivateKey>>,
}

#[derive(Clone, Debug)]
pub struct Connection {
    pub write_socket: Arc<Mutex<OwnedWriteHalf>>,
    pub read_socket: Arc<Mutex<OwnedReadHalf>>,
    pub agent_uuid: Uuid,
    pub shared_secret: [u8; 32],
}
