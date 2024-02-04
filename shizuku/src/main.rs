mod dbus;
mod widget;
use std::collections::HashMap;

use color_eyre::Result;
use gio::prelude::ApplicationExtManual;
use gtk::prelude::{GtkWindowExt, WidgetExt};
use tracing::{debug, trace, warn};

lazy_static::lazy_static! {
    static ref NOTIF_DESTROY_CHANS: std::sync::Arc<(async_std::channel::Sender<NotifStackEvent>, async_std::channel::Receiver<NotifStackEvent>)>
        = std::sync::Arc::new(async_std::channel::unbounded());
}

fn time_now() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards nya??")
        .as_millis()
}

#[derive(Default, Clone, Debug)]
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

    pub fn with_duration_secs(duration: usize) -> Self {
        Self {
            until: time_now() + (duration * 1000) as u128,
            duration,
        }
    }

    pub fn is_over(&self) -> bool {
        time_now() >= self.until
    }
}

#[derive(Debug, Clone)]
pub enum NotifStackEvent {
    Closed(u32), // notif id
    Added(widget::Notification),
}

#[derive(Clone, Default)]
pub struct NotificationStack(HashMap<u32, (widget::Notification, libhelium::Window)>);

impl NotificationStack {
    pub fn clear(&mut self) {
        self.0.clear();
    }

    #[tracing::instrument]
    fn on_post_close_notif(win: &libhelium::Window) {
        let window_name = win.widget_name();
        debug!(?window_name, "Notif window closed");

        // get window id by removing the "notif-" prefix
        let window_id = window_name.split_at(6).1.parse::<u32>().unwrap();

        let tx = NOTIF_DESTROY_CHANS.0.clone();

        win.close();

        async_std::task::spawn(async move {
            tx.send(NotifStackEvent::Closed(window_id)).await.unwrap();
        });
    }

    pub fn add(&mut self, mut notif: widget::Notification, app: &libhelium::Application) {
        let id = format!("notif-{}", notif.id);
        let span = tracing::debug_span!("add_notif", id);
        let _enter = span.enter();
        debug!("Adding new notif");
        let win = notif.as_window(app, self.0.len());
        win.set_widget_name(&id);
        // (notif.close_btn.as_ref())
        // .expect("No close button registered on notif")
        // .connect_clicked(Self::on_notif_close_btn_clicked);

        trace!("Setting window as visible");
        win.set_visible(true);
        win.connect_destroy(Self::on_post_close_notif);
        notif.sched = Some(NotifSchedTimer::new());
        trace!("Connected NotifSchedTimer");
        self.0.insert(notif.id, (notif, win));
    }

    #[tracing::instrument(skip(self))]
    pub fn poll(&mut self) -> Option<NotifStackEvent> {
        self.0
            .iter()
            .find(|(_, (notif, _))| notif.sched.as_ref().expect("No notif sched").is_over())
            .map(|(id, (notif, win))| {
                debug!(id, "Found notif that should be closed due to timeout!");
                win.close();
                NotifStackEvent::Closed(notif.id)
            })
    }

    #[tracing::instrument(skip(self))]
    pub fn remove(&mut self, index: u32) {
        debug!("Removing notif");
        self.0.remove(&index).map_or_else(
            || debug!("notif not found"),
            |(notif, win)| trace!(?notif, ?win, "notif removed"),
        );
    }

    pub fn get(&self, index: u32) -> Option<&widget::Notification> {
        self.0.get(&index).map(|obj| &obj.0)
    }
}

const APPLICATION_ID: &str = "com.fyralabs.shizuku";

#[derive(Clone)]
pub struct Application {
    pub app: libhelium::Application,
    pub stack: NotificationStack,
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}

impl Application {
    pub fn new() -> Self {
        let app = libhelium::Application::builder()
            .application_id(APPLICATION_ID)
            .flags(gio::ApplicationFlags::NON_UNIQUE)
            .build();

        let stack = NotificationStack::default();

        Self { app, stack }
    }

    pub fn run(&mut self) -> gtk::glib::ExitCode {
        let mut self_clone = self.clone();
        gtk::glib::MainContext::default().spawn_local(async move {
            self_clone.poll_msg_queue().await;
        });
        let _ = self.app.hold();
        self.app.run()
    }

    #[tracing::instrument(skip(self))]
    pub async fn poll_msg_queue(&mut self) {
        debug!("Polling for notif events");
        let rx = NOTIF_DESTROY_CHANS.1.clone();

        loop {
            if let Some(event) = self.stack.poll().or_else(|| rx.try_recv().ok()) {
                debug!(?event, "Processing event");

                match event {
                    NotifStackEvent::Closed(index) => {
                        self.stack.remove(index);
                    }
                    NotifStackEvent::Added(notif) => {
                        self.stack.add(notif, &self.app);
                    }
                }
            }
            // we don't want CPU 100% usage
            async_std::task::sleep(std::time::Duration::from_millis(50)).await;
        }
    }
}

fn main() -> Result<gtk::glib::ExitCode> {
    // dotenvy::dotenv()?;
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("SHIZUKU_LOG").unwrap_or_else(|_| "debug".to_string()),
        ))
        .pretty()
        .init();

    let mut application = Application::new();

    async_std::task::spawn(async {
        tracing::info!("Starting dbus server");
        let connection = zbus::Connection::session().await.unwrap();
        connection
            .object_server()
            .at(dbus::DBUS_OBJECT_PATH, dbus::NotificationsServer)
            .await
            .unwrap();

        connection.request_name(dbus::DBUS_INTERFACE).await.unwrap();

        std::future::pending::<()>().await;
    });

    // let application = libhelium::Application::builder()
    // .application_id("com.fyralabs.shizuku")
    // .build();

    // We hold the application here so that it doesn't exit immediately after the activate signal is emitted
    let _hold = application.app.hold();

    Ok(application.run())
}
