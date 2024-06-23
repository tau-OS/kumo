mod x11;
mod winit;
mod drm;

use smithay::backend::winit::WinitGraphicsBackend;


// #[derive(Debug)]
// pub struct Backend {
//     // pub event_loop: idaskiodksoiadioasdioasdj,
//     pub state: Box<BackendState>,

// }

#[derive(Debug)]
pub enum BackendState {
    Winit(winit::WinitBackend)
}
