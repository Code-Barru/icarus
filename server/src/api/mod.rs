use std::{net::SocketAddr, sync::Arc};

use tokio::sync::Mutex;
use tracing::error;

use crate::state::GlobalState;

pub struct WebServer {
    pub state: Arc<Mutex<GlobalState>>,
}

impl WebServer {
    pub fn new(state: GlobalState) -> Self {
        WebServer {
            state: Arc::new(Mutex::new(state)),
        }
    }

    pub async fn start_server(&self, addr: &SocketAddr) {
        let state = self.state.lock().await;
        let app = crate::utils::get_router(state.clone());

        let listener = match tokio::net::TcpListener::bind(addr).await {
            Ok(listener) => listener,
            Err(e) => {
                error!("Failed to bind: {}", e);
                return;
            }
        };

        match axum::serve(
            listener,
            app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
        )
        .await
        {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Server error: {}", e);
            }
        }
    }
}
