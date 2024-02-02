mod dbus;
mod widget;
use color_eyre::Result;
use gio::prelude::{ApplicationExt, ApplicationExtManual};
use gtk::prelude::{BoxExt, GtkWindowExt, WidgetExt};
// use gtk4_layer_shell::LayerShell;
use zbus::Connection;
use gtk4_layer_shell::{Edge, Layer, LayerShell};
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

/*     let connection = Connection::session().await?;
    connection
        .object_server()
        .at(dbus::DBUS_OBJECT_PATH, dbus::NotificationsServer)
        .await?;

    connection
        .request_name(dbus::DBUS_INTERFACE)
        .await?;

    loop {
        std::future::pending::<()>().await;
    } */



    let application = libhelium::Application::builder()
        .application_id("com.fyralabs.shizuku")
        .build();

    // todo: bind this code to an actual dbus server,
    // also, make them windows stackable so we can have multiple notifications
    application.connect_activate(|app| {
        let window = libhelium::Window::builder()
            .application(app)
            .title("Shizuku")
            // .default_width(500)
            // .default_height(100)
            .resizable(false)
            .decorated(false)
            .build();

        window.init_layer_shell();

        window.set_layer(Layer::Overlay);

        window.auto_exclusive_zone_enable();

        window.set_anchor(Edge::Top, true);
        window.set_anchor(Edge::Right, true);
        window.set_anchor(Edge::Bottom, false);
        window.set_anchor(Edge::Left, false);

        window.set_margin(Edge::Top, 15);
        window.set_margin(Edge::Right, 15);




        let box_ = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(10)
            .build();

        // let label = gtk::Label::builder()
        //     .label("Hello, world!")
        //     .build();

        // box_.append(&label);

        let notification = widget::Notification::new(
            "Hello, world!".to_string(),
            "This is a test notification".to_string(),
            Some("dialog-information".to_string()),
            dbus::Urgency::Low,
            0,
        );

        let notification_box = notification.as_box();

        box_.append(&notification_box);


        // remove the box later

        // box_.remove(&notification_box);

        window.set_child(Some(&box_));

        window.show();


    });

    application.run();

    Ok(())

}