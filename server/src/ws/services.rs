use serde_json::Value;
use socketioxide::{
    extract::{Data, SocketRef},
    SocketIo,
};
use tracing::info;

fn on_connect(socket: SocketRef) {
    info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);

    socket.on("message", |_socket: SocketRef, Data::<Value>(data)| {
        info!("Socket.IO message: {:?}", data);
    });
}

pub fn setup_ws(io: &SocketIo) {
    io.ns("/", move |socket| on_connect(socket));
}
