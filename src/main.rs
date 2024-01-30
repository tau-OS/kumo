mod wf_ipc;
mod wf_socket;
use color_eyre::Result;
use serde_json::json;
fn main() -> Result<()> {
    dotenvy::dotenv()?;

    color_eyre::install()?;

    tracing_subscriber::fmt::init();

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
