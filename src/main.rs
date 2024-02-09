use color_eyre::Result;
fn main() -> Result<()> {
    dotenvy::dotenv()?;

    color_eyre::install()?;

    // set envar for log to KUMO_LOG inst6ead of RUST_LOG
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("KUMO_LOG").unwrap_or_else(|_| "info".to_string()),
        ))
        .init();

    Ok(())
}
