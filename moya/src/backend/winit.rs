use calloop::EventLoop;
use smithay::backend::winit::{self, WinitEvent, WinitGraphicsBackend, WinitVirtualDevice};
use smithay::{
    backend::renderer::{element::surface::WaylandSurfaceRenderElement, glow::GlowRenderer},
    output::{Mode, Output, PhysicalProperties, Subpixel},
    reexports::calloop::timer::TimeoutAction,
    utils::{Rectangle, Transform},
};
#[derive(Debug)]
pub struct WinitBackend {
    pub output: Output,
}

impl Default for WinitBackend {
    fn default() -> Self {
        Self::init()
    }
}

impl WinitBackend {
    pub fn init() -> Self {
        let output = Output::new(
            "winit".to_string(),
            PhysicalProperties {
                size: (0, 0).into(),
                subpixel: Subpixel::Unknown,
                make: "Moya".to_string(),
                model: "Winit".to_string(),
            },
        );

        // ref https://github.com/StrataWM/strata/blob/5daa4f102a7a03bb73dbe84e43d7ae1cb64d2c54/src/backends/winit.rs#L49

        let mut event_loop: EventLoop<crate::state::MoyaState> = EventLoop::try_new().unwrap();

        todo!()
    }
}
