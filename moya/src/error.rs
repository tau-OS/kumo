use wayland_server::{
    backend::{ClientData, ClientId, DisconnectReason},
    Display,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Client (id {0:?}) disconnected with wayland: {1:?}")]
    WaylandDisconnected(ClientId, DisconnectReason),
}
