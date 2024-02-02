mod dbus;
mod widget;
use std::sync::Mutex;

use color_eyre::Result;
use gio::prelude::{ApplicationExt, ApplicationExtManual};
use gtk::prelude::{BoxExt, GtkWindowExt, WidgetExt};
// use gtk4_layer_shell::LayerShell;
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use zbus::Connection;

lazy_static::lazy_static! {
    static ref NOTIF_STACK: Mutex<NotificationStack> = Mutex::new(NotificationStack::new(vec![]));
}

pub struct NotificationStack {
    pub notifications: Vec<widget::Notification>,
}

impl NotificationStack {
    pub fn new(notifications: Vec<widget::Notification>) -> Self {
        Self { notifications }
    }

    pub fn clear(&mut self) {
        self.notifications.clear();
    }

    pub fn add(&mut self, notif: widget::Notification, app: &libhelium::Application) {
        self.notifications.push(notif);

        // then get the count of all notifications
        let count = self.notifications.len();

        // now show with index
        self.notifications
            .last_mut()
            .unwrap()
            .as_window(app, count - 1)
            .set_visible(true);
    }
}

// a basic listener for now, prints notifications to stdout using println!
fn main() -> Result<gtk::glib::ExitCode> {
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
        let notification = widget::Notification::new(
            "Hello".to_string(),
            "This is a notification".to_string(),
            None,
            dbus::Urgency::Low,
            0,
        );

        let notif2 = widget::Notification::new(
            "Hello".to_string(),
            "This is a notification too".to_string(),
            None,
            dbus::Urgency::Low,
            0,
        );

        let mut notifications = vec![notification, notif2];

        for (i, notif) in notifications.iter_mut().enumerate() {
            let win = notif.as_window(app, i);

            // spawn thread

            gtk::glib::spawn_future_local(async move {
                win.show();
            });

            // show window for 5 seconds and then close it

            // win.connect_activate_default(move |window| {

            //     println!("Window activated");
            //     // wait for 5 seconds and then close the window

            //     // std::thread::sleep(std::time::Duration::from_secs(5));

            //     window.set_visible(false);
            //     println!("Closing window");
            // });

            // win.activate();

            // println!("Window shown");
        }
    });

    let hold = application.hold();

    Ok(application.run())
}
