use serde_json::Value;
use socketioxide::{
    extract::{Data, SocketRef},
    layer::SocketIoLayer,
    SocketIo,
};
use tracing::info;

use crate::AppState;

fn on_connect(socket: SocketRef, state: AppState) {
    info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);

    socket.on("message", |_socket: SocketRef, Data::<Value>(data)| {
        info!("Socket.IO message: {:?}", data);
    });
}

pub fn get_layer(state: AppState) -> SocketIoLayer {
    let (layer, io) = SocketIo::new_layer();
    io.ns("/", move |socket| on_connect(socket, state.clone()));
    layer
}
