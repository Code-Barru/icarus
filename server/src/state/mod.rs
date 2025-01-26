use std::sync::Arc;
use tokio::sync::Mutex;

pub struct GlobalState {
    pg_connection: Arc<Mutex<diesel::PgConnection>>,
}

impl GlobalState {
    pub fn new(pg_connection: diesel::PgConnection) -> Self {
        GlobalState {
            pg_connection: Arc::new(Mutex::new(pg_connection)),
        }
    }
}
