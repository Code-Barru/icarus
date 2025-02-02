#![allow(dead_code)]
mod agent;
mod connection;

use crate::live::Connection;
use crate::utils::establish_connection;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct GlobalState {
    pg_connection: Arc<Mutex<diesel::PgConnection>>,
    connections: Arc<Mutex<Vec<Connection>>>,
}

impl GlobalState {
    pub fn new() -> Self {
        let pg_connection = establish_connection();
        GlobalState {
            pg_connection: Arc::new(Mutex::new(pg_connection)),
            connections: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
