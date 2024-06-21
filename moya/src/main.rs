use clap::Parser;
use color_eyre::Result;

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

    // todo: dedicated logger struct
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("KUMO_LOG").unwrap_or_else(|_| "info".to_string()),
        ))
        .init();

    MoyaLauncher::parse(); // todo: implement

    Ok(())
}
