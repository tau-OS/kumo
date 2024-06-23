use smithay::{
    delegate_output,
    desktop::{PopupManager, Space, Window},
    input::{Seat, SeatHandler, SeatState},
    wayland::{
        compositor::CompositorState, selection::data_device::DataDeviceState,
        shell::xdg::XdgShellState, shm::ShmState,
    },
};
use tracing::instrument;
use wayland_server::{
    backend::{ClientData, ClientId, DisconnectReason},
    Display,
};

#[derive(Debug)]
pub struct MoyaState {
    pub backend_state: crate::backend::BackendState,
    pub xdg_shell: XdgShellState,
    pub compositor: CompositorState,
    pub shm: ShmState,
    pub data_device: DataDeviceState,
    pub seats: Vec<Seat<Self>>,
}

impl From<crate::MoyaBackend> for MoyaState {
    fn from(value: crate::MoyaBackend) -> Self {
        Self {
            backend_state: match value {
                crate::MoyaBackend::Winit => {
                    crate::backend::BackendState::Winit(Default::default())
                }
                crate::MoyaBackend::X11 => todo!(),
                crate::MoyaBackend::Udev => todo!(),
            },
            xdg_shell: todo!(),
            compositor: todo!(),
            shm: todo!(),
            data_device: todo!(),
            seats: todo!(),
        }
    }
}

// impl MoyaState {
//     pub fn create_backend(backend: crate::MoyaBackend) -> Self {
//         todo!()
//     }
// }

impl SeatHandler for MoyaState {
    type KeyboardFocus;

    type PointerFocus;

    type TouchFocus;

    fn seat_state(&mut self) -> &mut SeatState<Self> {
        todo!()
    }
}
