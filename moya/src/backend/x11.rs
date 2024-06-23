// todo: implement X11 backend
use color_eyre::eyre::Context;
use smithay::{
    backend::{
        allocator::gbm::{GbmAllocator, GbmDevice},
        drm::{DrmDeviceFd, DrmNode},
        egl::{EGLContext, EGLDevice, EGLDisplay},
        renderer::glow::GlowRenderer,
        vulkan::{version::Version, Instance, PhysicalDevice},
        x11::{WindowBuilder, X11Handle},
    },
    output::Output,
    reexports::gbm::Surface,
    utils::DeviceFd,
};
use tracing::{debug, error};

#[derive(Debug)]
pub enum X11Allocator {
    Gbm(GbmAllocator<DrmDeviceFd>),
    Vulkan(PhysicalDevice),
}

impl Default for X11Allocator {
    fn default() -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub struct MoyaX11 {
    pub allocator: X11Allocator,
    _egl: EGLDisplay,
    pub renderer: GlowRenderer,
    surfaces: Vec<Surface<()>>, // fixme
    pub handle: X11Handle,
}

impl Default for MoyaX11 {
    fn default() -> Self {
        let backend =
            smithay::backend::x11::X11Backend::new().expect("Failed to initilize X11 backend");
        let handle = backend.handle();

        // Obtain the DRM node the X server uses for direct rendering.
        let (drm_node, fd) = handle
            .drm_node()
            .expect("Could not get DRM node used by X server");
        let device = EGLDevice::enumerate()
            .expect("Failed to enumerate EGL devices")
            .find(|device| device.try_get_render_node().ok().flatten() == Some(drm_node))
            .expect(&format!("Failed to find EGLDevice for node {drm_node}"));
        let _egl =
            unsafe { EGLDisplay::new(device.clone()) }.expect("Failed to create EGL display");
        let context = EGLContext::new(&_egl).expect("Failed to create EGL context");
        Self {
            allocator: Default::default(),
            _egl,
            renderer: unsafe { GlowRenderer::new(context) }.expect("Failed to initialize renderer"),
            surfaces: Default::default(),
            handle,
        }
    }
}

// ref https://github.com/pop-os/cosmic-comp/blob/master/src/backend/x11.rs#L50
impl MoyaX11 {
    pub fn new_window(&mut self) -> color_eyre::Result<Output> {
        let window = WindowBuilder::new()
            .title("Moya")
            .build(&self.handle)
            .with_context(|| "Failed to create window")?;

        todo!()
    }
}

impl super::BackendInterface for MoyaX11 {}

fn try_gbm_allocator(fd: std::os::fd::OwnedFd) -> Option<X11Allocator> {
    // Create the gbm device for buffer allocation.
    let device = match GbmDevice::new(DrmDeviceFd::new(DeviceFd::from(fd))) {
        Ok(gbm) => gbm,
        Err(err) => {
            error!(?err, "Failed to create GBM device.");
            return None;
        }
    };

    Some(X11Allocator::Gbm(GbmAllocator::new(
        device,
        smithay::reexports::gbm::BufferObjectFlags::RENDERING,
    )))
}

// FIXME?
fn try_vulkan_allocator(node: &DrmNode) -> Option<X11Allocator> {
    let instance = match Instance::new(Version::VERSION_1_2, None) {
        Ok(instance) => instance,
        Err(err) => {
            tracing::warn!(
                ?err,
                "Failed to instanciate vulkan, falling back to gbm allocator.",
            );
            return None;
        }
    };

    let devices = match PhysicalDevice::enumerate(&instance) {
        Ok(devices) => devices,
        Err(err) => {
            debug!(?err, "No vulkan devices, falling back to gbm.");
            return None;
        }
    };

    let Some(device) = devices
        .filter(|phd| {
            phd.has_device_extension(
                // smithay::reexports::ash::extensions::ext::PhysicalDeviceDrm::name(),
                todo!(),
            )
        })
        .find(|phd| {
            phd.primary_node().unwrap() == Some(*node) || phd.render_node().unwrap() == Some(*node)
        })
    else {
        debug!(?node, "No vulkan device for node, falling back to gbm.");
        return None;
    };

    Some(X11Allocator::Vulkan(device))
}
