mod dbus;
use color_eyre::Result;
use zbus::Connection;
// a basic listener for now, prints notifications to stdout using println!
#[tokio::main]
async fn main() -> Result<()> {
    // dotenvy::dotenv()?;

    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("SHIZUKU_LOG").unwrap_or_else(|_| "debug".to_string()),
        ))
        .pretty()
        .init();

    // establish as dbus server

    let connection = Connection::session().await?;
    connection
        .object_server()
        .at(dbus::DBUS_OBJECT_PATH, dbus::NotificationsServer)
        .await?;

    connection
        .request_name(dbus::DBUS_INTERFACE)
        .await?;

    loop {
        std::future::pending::<()>().await;
    }

    // Ok(())
}
