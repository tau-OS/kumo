use std::cell::RefCell;

use calloop::{ping, EventLoop};
use smithay::backend::renderer::damage::OutputDamageTracker;
use smithay::backend::winit::{self, WinitEvent, WinitGraphicsBackend, WinitVirtualDevice};
use smithay::output::Scale;
use smithay::{
    backend::renderer::{
        element::surface::WaylandSurfaceRenderElement, glow::GlowRenderer, multigpu::MultiRenderer,
    },
    output::{Mode, Output, PhysicalProperties, Subpixel},
    reexports::calloop::timer::TimeoutAction,
    utils::{Rectangle, Transform},
};
// ref https://github.com/pop-os/cosmic-comp/blob/254c583b5dc1c9435a51d1817facb1f0c2125637/src/backend/winit.rs#L43
#[derive(Debug)]
pub struct WinitBackend {
    pub output: Output,
    pub graphics_backend: WinitGraphicsBackend<GlowRenderer>, // or MultiRenderer or rawdog GLES
    damage_tracker: OutputDamageTracker,
}
// https://www.x.org/releases/X11R7.5/doc/damageproto/damageproto.txt
impl Default for WinitBackend {
    fn default() -> Self {
        Self::init()
    }
}

impl super::BackendInterface for WinitBackend {}

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

        // todo!();

        let (mut backend, mut input) =
            smithay::backend::winit::init().expect(&format!("Failed to initilize winit backend"));
        // init_shaders(backend.renderer()).context("Failed to initialize renderer")?;

        // init_egl_client_side(dh, state, &mut backend)?;

        let name = format!("WINIT-0");
        let size = backend.window_size();
        let props = PhysicalProperties {
            size: (0, 0).into(),
            subpixel: Subpixel::Unknown,
            make: "COSMIC".to_string(),
            model: name.clone(),
        };
        let mode = Mode {
            size: (size.w as i32, size.h as i32).into(),
            refresh: 60_000,
        };
        let output = Output::new(name, props);
        output.add_mode(mode);
        output.set_preferred(mode);
        output.change_current_state(
            Some(mode),
            Some(Transform::Flipped180), // ← !?
            Some(Scale::Integer(1)),
            Some((0, 0).into()),
        );
        // output.user_data().insert_if_missing(|| {
        //     RefCell::new(OutputConfig {
        //         mode: ((size.w as i32, size.h as i32), None),
        //         transform: Transform::Flipped180.into(),
        //         ..Default::default()
        //     })
        // });

        let (event_ping, event_source) =
            ping::make_ping().expect("Failed to init eventloop timer for winit");
        let (render_ping, render_source) =
            ping::make_ping().expect("Failed to init eventloop timer for winit"); // expect is fine for now
        let event_ping_handle = event_ping.clone();
        let render_ping_handle = render_ping.clone();
        // let mut token = Some(
        //     event_loop
        //         .handle()
        //         .insert_source(render_source, move |_, _, state| {
        //             if let Err(err) = state.backend.winit().render_output(&mut state.common) {
        //                 error!(?err, "Failed to render frame.");
        //                 render_ping.ping();
        //             }
        //             profiling::finish_frame!();
        //         })
        //         .map_err(|_| anyhow::anyhow!("Failed to init eventloop timer for winit"))?,
        // );
        let event_loop_handle = event_loop.handle();
        // event_loop
        //     .handle()
        //     .insert_source(event_source, move |_, _, state| {
        //         match input.dispatch_new_events(|event| {
        //             state.process_winit_event(event, &render_ping_handle)
        //         }) {
        //             PumpStatus::Continue => {
        //                 event_ping_handle.ping();
        //                 render_ping_handle.ping();
        //             }
        //             PumpStatus::Exit(_) => {
        //                 let output = state.backend.winit().output.clone();
        //                 state.common.remove_output(&output);
        //                 if let Some(token) = token.take() {
        //                     event_loop_handle.remove(token);
        //                 }
        //             }
        //         };
        //     })
        //     .map_err(|_| anyhow::anyhow!("Failed to init eventloop timer for winit"))?;
        event_ping.ping();
        // WinitState {
        //     backend,
        //     output: output.clone(),
        //     damage_tracker: OutputDamageTracker::from_output(&output),
        //     fps,
        // }
        Self {
            output,
            graphics_backend: todo!(),
            damage_tracker: todo!(),
        }
    }
}
