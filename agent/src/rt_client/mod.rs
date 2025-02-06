use std::sync::Arc;
use tokio::{
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
    sync::Mutex,
};

pub mod handshake;
pub mod packet_handler;
pub mod rt_client;
pub mod update;

#[derive(Clone, Debug)]
pub struct RTClient {
    write_socket: Arc<Mutex<OwnedWriteHalf>>,
    read_socket: Arc<Mutex<OwnedReadHalf>>,
    shared_secret: Arc<Mutex<[u8; 32]>>,
}
