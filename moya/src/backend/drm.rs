//! DRM backend using udev
//!
//! Note for non techies: This DRM stands for "Direct Rendering Manager", not "Digital Rights Management"
//! It's a Linux kernel subsystem for rendering graphics, and it's what powers the Linux desktop.
//!
//! Digital Rights Management is a different thing which is probably implemented on a higher level or literally on hardware.
//!
//! Ask Mesa or your userspace app for that DRM I guess. HDMI 2.1 isn't supported thanks to the forum hating AMD.
//!

// is there really no vulkan renderer? what.
// https://github.com/Smithay/smithay/issues/134
// okay, https://github.com/ash-rs/ash is a thing, we import that already
use smithay::backend::vulkan::PhysicalDevice;

// oh my fucking god. https://github.com/pop-os/cosmic-comp/blob/254c583b5dc1c9435a51d1817facb1f0c2125637/src/backend/kms/drm_helpers.rs#L237C2-L276C29

/// EDID metadata for displays
pub struct EdidInfo {
    pub model: String,
    pub manufacturer: String,
}

/*
#[derive(Debug)]
pub struct Surface {
    surface: Option<GbmDrmCompositor>,
    connector: connector::Handle,

    output: Output,
    mirroring: Option<Output>,
    mirroring_textures: HashMap<DrmNode, MirroringState>,

    refresh_rate: u32,
    vrr: bool,
    scheduled: bool,
    pending: bool,
    render_timer_token: Option<RegistrationToken>,
    fps: Fps,
    feedback: HashMap<DrmNode, SurfaceDmabufFeedback>,
} */

// fuck it, do GBM for now

// ref https://github.com/pop-os/cosmic-comp/blob/master/src/backend/kms/mod.rs
// ref https://github.com/YaLTeR/niri/blob/main/src/backend/tty.rs

// todo: Implement KMS renderers, preferrably Vulkan, then EGL/GBM/OpenGL
// Trait for KMS renderers, I guess :P
pub trait DrmRenderer {}

#[derive(Debug, Default)]
pub struct DrmBackend {}

impl super::BackendInterface for DrmBackend {
    fn event_loop(&self) -> &calloop::LoopHandle<'static, Self>
        where
            Self: Sized {
        
        todo!()
    }
}
