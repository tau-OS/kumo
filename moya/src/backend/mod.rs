pub mod drm;
pub mod winit;
pub mod x11;

use calloop::LoopHandle;

// i have no idea what data to pass for each backend

#[derive(Debug)]
pub struct Backend {
    pub event_loop: LoopHandle<'static, BackendState>,
}

#[derive(Debug)]
pub enum BackendState {
    Winit(winit::WinitBackend),
    X11(x11::MoyaX11),
    Drm(drm::DrmBackend),
}

pub trait BackendInterface {
    fn event_loop(&self) -> &LoopHandle<'static, Self>
    where
        Self: Sized;

    fn dispatch(&mut self) {
        todo!()
    }
}

// turning stuff to dyn so we don't need to write implementation 3 times
// Helper functions for converting memory references
impl BackendState {
    pub fn as_dyn(&self) -> &dyn BackendInterface {
        match self {
            BackendState::Winit(x) => x,
            BackendState::X11(x) => x,
            BackendState::Drm(x) => x,
        }
    }
    pub fn as_mut_dyn(&mut self) -> &mut dyn BackendInterface {
        match self {
            BackendState::Winit(x) => x,
            BackendState::X11(x) => x,
            BackendState::Drm(x) => x,
        }
    }
    pub fn do_ref<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&dyn BackendInterface) -> T,
    {
        f(self.as_dyn())
    }
    pub fn do_mut<T, F>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut dyn BackendInterface) -> T,
    {
        f(self.as_mut_dyn())
    }


}

impl From<crate::MoyaBackend> for BackendState {
    fn from(value: crate::MoyaBackend) -> Self {
        match value {
            crate::MoyaBackend::Winit => Self::Winit(Default::default()),
            crate::MoyaBackend::X11 => Self::X11(Default::default()),
            crate::MoyaBackend::Udev => Self::Drm(Default::default()),
        }
    }
}
