use smithay::{
    delegate_output,
    desktop::{LayerSurface, PopupKind, PopupManager, Space, Window},
    input::{Seat, SeatHandler, SeatState},
    wayland::{
        compositor::CompositorState, selection::data_device::DataDeviceState, session_lock::LockSurface, shell::xdg::{decoration::XdgDecorationState, XdgShellState}, shm::ShmState
    },
};
use tracing::instrument;
use wayland_server::{
    backend::{ClientData, ClientId, DisconnectReason}, protocol::wl_surface::WlSurface, Display
};

// ref https://github.com/pop-os/cosmic-comp/blob/master/src/state.rs
// ref https://github.com/pop-os/cosmic-comp/blob/master/src/wayland/handlers/seat.rs

use crate::MoyaBackend;
// hello from tree
#[derive(Debug)]
pub struct XdgShellHandler {
    pub state: XdgShellState,
    pub decorations: XdgDecorationState,
}

/// Current compositor state, should be used to pass around
#[derive(Debug)]
pub struct MoyaState {
    // We probably don't want backend state to be inside tis
    pub backend_state: crate::backend::BackendState,
    pub xdg_shell: XdgShellHandler,
    pub compositor: CompositorState,
    pub shm: ShmState,
    pub data_device: DataDeviceState,
    // pub seats: Vec<Seat<Self>>,
    pub space: Space<Window>,
    pub popup_manager: PopupManager,
    pub seat: SeatState<Self> // todo: Implement 
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeyboardFocusTarget {
    // Element(CosmicMapped),
    // Fullscreen(CosmicSurface),
    // Group(WindowGroup),
    LayerSurface(LayerSurface),
    Popup(PopupKind),
    LockSurface(LockSurface),
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
            // seats: todo!(),

            popup_manager: todo!(),
            seat: todo!(),
            space: todo!(),
        }
    }
}
// todo: fix!
impl MoyaState {
    pub fn new(display: &mut Display<Self>, backend: MoyaBackend) -> Self {
        let display_handle = display.handle();
        let mut seat_state: SeatState<Self> = SeatState::new();
        let seat = SeatState::new();
        Self {
            backend_state: match backend {
                crate::MoyaBackend::Winit => {
                    crate::backend::BackendState::Winit(Default::default())
                }
                crate::MoyaBackend::X11 => todo!(),
                crate::MoyaBackend::Udev => todo!(),
            },
            compositor: todo!("CompositorState::new(&display_handle)"),
            shm: ShmState::new(&display_handle, vec![]),
            data_device: DataDeviceState::new(&display_handle),
            xdg_shell: XdgShellHandler {
                state: XdgShellState::new(&display_handle),
                decorations: XdgDecorationState::new(&display_handle),
            },
            space: Space::default(),
            popup_manager: PopupManager::default(),
            // backend_state: todo!()
            seat
        }
    }
}

delegate_output!(MoyaState);

// impl MoyaState {
//     pub fn create_backend(backend: crate::MoyaBackend) -> Self {
//         todo!()
//     }
// }

// todo: actually implement seat handler trait properly for real global dispatch bullshittery

// impl SeatHandler for MoyaState {
//     type KeyboardFocus = WlSurface;

//     type PointerFocus = WlSurface;

//     type TouchFocus = WlSurface;

//     fn seat_state(&mut self) -> &mut SeatState<Self> {
//         todo!()
//     }
// }
