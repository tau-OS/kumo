mod dbus;
mod widget;
use color_eyre::Result;
use gio::{
    glib::Cast,
    prelude::{ApplicationExt, ApplicationExtManual},
};
use gtk::prelude::{ButtonExt, GtkWindowExt, WidgetExt};
use std::{
    sync::mpsc,
    time::{SystemTime, UNIX_EPOCH},
};
use tracing::warn;

lazy_static::lazy_static! {
    static ref NOTIF_DESTROY_CHANS: std::sync::Arc<(async_std::channel::Sender<NotifStackEvent>, async_std::channel::Receiver<NotifStackEvent>)> = std::sync::Arc::new(async_std::channel::unbounded());
}

fn time_now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards nya??")
        .as_millis()
}

#[derive(Default, Clone)]
pub struct NotifSchedTimer {
    pub until: u128,     // scheduled unix time in ms to hide the notif
    pub duration: usize, // duration of notif on screen in secs
}

impl NotifSchedTimer {
    pub fn new() -> Self {
        Self {
            until: time_now() + 10 * 1000,
            duration: 10,
        }
    }

    pub fn is_over(&self) -> bool {
        time_now() >= self.until
    }
}

pub enum NotifStackEvent {
    Closed(usize), // pos of notif in the stack vec
}

pub struct NotificationStack(Vec<widget::Notification>);

impl NotificationStack {
    pub fn new(notifications: Vec<widget::Notification>) -> Self {
        Self { 0: notifications }
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    fn post_close_notif(&mut self, index: usize) {
        self.0.remove(index);
    }

    fn on_notif_close_btn_clicked(btn: &gtk::Button) {
        btn.parent()
            .and_then(|action| action.parent())
            .and_then(|box_| box_.parent())
            .map(|widget| widget.downcast::<libhelium::Window>())
            .map_or_else(
                || warn!("Can't get parent in on_close_btn_clicked()"),
                |window| window.expect("fail to downcast").close(),
            );
    }

    fn on_post_close_notif(win: &libhelium::Window) {
        todo!()
    }

    pub fn add(&mut self, mut notif: widget::Notification, app: &libhelium::Application) {
        let win = notif.as_window(app, self.0.len());
        (notif.close_btn.as_ref())
            .expect("No close button registered on notif")
            .connect_clicked(Self::on_notif_close_btn_clicked);

        win.set_visible(true);
        win.connect_destroy(Self::on_post_close_notif);
        notif.sched = Some(NotifSchedTimer::new());
        self.0.push(notif);
    }

    pub fn poll(&mut self) {
        let mut notifs_to_rm = vec![];
        for (i, notif) in self.0.iter().enumerate() {
            if notif.sched.as_ref().expect("No notif sched").is_over() {
                // pretend we're closing it
                Self::on_notif_close_btn_clicked(&notif.close_btn.as_ref().expect("No close btn"));
                notifs_to_rm.push(i);
            }
        }
        (notifs_to_rm.into_iter().enumerate()).for_each(|(n, i)| self.post_close_notif(i - n));
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
        let mut stack = NotificationStack::new(vec![]);
        stack.add(
            widget::Notification::new(
                "Hello".to_string(),
                "This is a notification".to_string(),
                None,
                dbus::Urgency::Low,
                0,
            ),
            app,
        );
        stack.add(
            widget::Notification::new(
                "Hello".to_string(),
                "This is a notification too".to_string(),
                None,
                dbus::Urgency::Low,
                0,
            ),
            app,
        );

        //     for (i, notif) in notifications.iter_mut().enumerate() {
        //         let win = notif.as_window(app, i);
        //
        //         // spawn thread
        //
        //         gtk::glib::spawn_future_local(async move {
        //             win.show();
        //         });
        //
        //         // show window for 5 seconds and then close it
        //
        //         // win.connect_activate_default(move |window| {
        //
        //         //     println!("Window activated");
        //         //     // wait for 5 seconds and then close the window
        //
        //         //     // std::thread::sleep(std::time::Duration::from_secs(5));
        //
        //         //     window.set_visible(false);
        //         //     println!("Closing window");
        //         // });
        //
        //         // win.activate();
        //
        //         // println!("Window shown");
        //     }
    });

    let hold = application.hold();

    Ok(application.run())
}
