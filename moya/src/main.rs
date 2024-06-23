#![warn(clippy::complexity)]
#![warn(clippy::correctness)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::suspicious)]
// followings are from clippy::restriction
#![warn(clippy::missing_errors_doc)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::missing_safety_doc)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]
#![warn(clippy::format_push_string)]
#![warn(clippy::get_unwrap)]
#![allow(clippy::missing_inline_in_public_items)]
#![allow(clippy::implicit_return)]
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::pattern_type_mismatch)]
use backend::winit;
use clap::Parser;
use color_eyre::Result;
pub mod backend;
pub mod error;
pub mod input;
mod logger;
pub mod state;
pub mod ui;

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum MoyaBackend {
    Winit,
    X11,
    Udev,
}

/// Returns the default backend depending on what's available right now.
#[must_use]
pub fn default_backend() -> MoyaBackend {
    if std::env::var_os("DISPLAY").is_some() || std::env::var_os("WAYLAND_DISPLAY").is_some() {
        MoyaBackend::Winit
    } else {
        MoyaBackend::Udev
    }
}

// CLI launcher for compositor
/// Moya is a Wayland compositor for Linux, written in Rust.
/// It is part of the KIRI desktop environment.
#[derive(Parser)]
pub struct MoyaLauncher {
    /// Rendering backend to use
    #[clap(
        short = 'B',
        long,
        // default_with = "default_backend",
        default_value = "winit",
        env = "MOYA_BACKEND"
    )]
    backend: MoyaBackend,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    crate::logger::init();

    let m = MoyaLauncher::parse(); // todo: implement


    let backend_state = backend::BackendState::from(m.backend);

    // let event_loop = 

    

    // something something



    Ok(())
}
