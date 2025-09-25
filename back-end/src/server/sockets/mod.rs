pub mod join;
pub mod message;

use socketioxide::extract::SocketRef;

pub fn connect(socket: &SocketRef) {
    tracing::info!("client {} connected", socket.id);
}

pub fn disconnect(socket: SocketRef) {
    tracing::info!("client {} disconnected", socket.id);
}
