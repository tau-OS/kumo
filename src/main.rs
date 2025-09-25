use std::sync::OnceLock;

use stable_eyre::eyre::Result;
use tokio::runtime::Runtime;
mod app;
mod components;
mod util;
#[must_use]
fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."))
}

fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    stable_eyre::install()?;

    // set envar for log to KUMO_LOG inst6ead of RUST_LOG
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("KUMO_LOG").unwrap_or_else(|_| "info".to_string()),
        ))
        .init();
    // let file = std::path::PathBuf::from("/usr/share/applications/Alacritty.desktop");
    // let a = util::gio_launch_desktop_file(&file).unwrap();
    //
    // println!("a: {:?}", a);

    // util::launch_desktop("btop").unwrap();

    let app = app::Application::new()?;
    app.run();
    Ok(())
}
