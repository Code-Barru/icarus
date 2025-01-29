mod agent;

use crate::utils::establish_connection;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};

#[derive(Clone)]
pub struct GlobalState {
    pg_connection: Arc<Mutex<diesel::PgConnection>>,
}

impl GlobalState {
    pub fn new() -> Self {
        let pg_connection = establish_connection();
        GlobalState {
            pg_connection: Arc::new(Mutex::new(pg_connection)),
        }
    }

    pub async fn get_conn(&self) -> MutexGuard<'_, diesel::PgConnection> {
        self.pg_connection.lock().await
    }
}
