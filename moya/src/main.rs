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
use clap::Parser;
use color_eyre::Result;
pub mod backend;
pub mod ui;
mod logger;
pub mod input;
pub mod state;


#[derive(Debug, Clone, clap::ValueEnum)]
pub enum MoyaBackend {
    Winit,
    X11,
    Udev,
}

// CLI launcher for compositor
/// Moya is a Wayland compositor for Linux, written in Rust.
/// It is part of the KIRI desktop environment.
#[derive(Parser)]
pub struct MoyaLauncher {
    /// Rendering backend to use
    #[clap(short = 'B', long, default_value = "winit")]
    backend: MoyaBackend,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    crate::logger::init();

    MoyaLauncher::parse(); // todo: implement

    Ok(())
}
