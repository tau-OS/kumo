mod wf_ipc;
mod wf_socket;
mod wf_shell_proto;
use color_eyre::Result;
use serde_json::json;
fn main() -> Result<()> {
    dotenvy::dotenv()?;

    color_eyre::install()?;

    // set envar for log to KUMO_LOG inst6ead of RUST_LOG
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("KUMO_LOG").unwrap_or_else(|_| "info".to_string()),
        ))
        .init();

    let socket = wf_socket::get_wayfire_socket();
    tracing::info!(?socket, "Wayfire Socket");

    let mut socket = wf_socket::WayfireSocket::new();

    loop {
        tracing::info!("Sent watch command");
        socket.send_json(json!(
            {
                "method": "window-rules/events/watch",
                "data": {}
            }
        ))?;

        let response = socket.read_message()?;

        tracing::info!(?response, "Received response");
    }

    Ok(())
}
