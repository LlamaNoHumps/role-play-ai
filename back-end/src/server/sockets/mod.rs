pub mod join;
pub mod message;
pub mod voice;

use socketioxide::extract::SocketRef;

pub fn connect(socket: &SocketRef) {
    let sid = socket.id;

    socket.join(sid);

    tracing::info!("client {} connected", sid);
}

pub fn disconnect(socket: SocketRef) {
    tracing::info!("client {} disconnected", socket.id);
}
