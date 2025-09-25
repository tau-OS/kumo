use color_eyre::Result;
mod app;
mod util;
fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    color_eyre::install()?;

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

    util::launch_desktop("btop").unwrap();
    Ok(())
}
