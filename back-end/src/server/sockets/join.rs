use socketioxide::extract::{Data, SocketRef};

pub const EVENT: &str = "join";

pub fn handler(socket: SocketRef, Data(room): Data<String>) {
    socket.join(room.clone());

    tracing::info!("client {} joined room {}", socket.id, room);
}
