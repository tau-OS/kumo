mod dbus;
mod widget;

use color_eyre::Result;
use gio::prelude::ApplicationExtManual;
use glib::{translate::FromGlib, ObjectExt};
use gtk::prelude::{GtkWindowExt, WidgetExt};
use std::collections::HashMap;
use tracing::{debug, trace, warn};

const APPLICATION_ID: &str = "com.fyralabs.shizuku";
const NO_LOG_ENV_MSG: &str = "Logging fallback as debug as env `SHIZUKU_LOG` is undefined. See https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives";
/// Max duration of notification in secs
const MAX_DURATION: u128 = 10;
lazy_static::lazy_static! {
    static ref NOTIF_CHANS: std::sync::Arc<(async_std::channel::Sender<NotifStackEvent>, async_std::channel::Receiver<NotifStackEvent>)>
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
    pub until: u128,    // scheduled unix time in ms to hide the notif
    pub duration: u128, // duration of notif on screen in secs
}

impl NotifSchedTimer {
    pub fn new() -> Self {
        Self {
            until: time_now() + MAX_DURATION * 1000,
            duration: MAX_DURATION,
        }
    }

    pub fn with_duration_secs(duration: u128) -> Self {
        let duration = duration.min(MAX_DURATION);
        Self {
            until: time_now() + duration * 1000,
            duration,
        }
    }

    #[inline]
    pub fn is_over(&self) -> bool {
        time_now() >= self.until
    }
}

#[derive(Debug, Clone)]
pub enum NotifStackEvent {
    Closed(u32), // notif id
    Added(widget::Notification),
}

/// A HashMap of notif ids and ([widget::Notification], [libhelium::Window]).
#[derive(Clone, Default)]
pub struct NotificationStack(HashMap<u32, (widget::Notification, libhelium::Window)>);

impl NotificationStack {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    // FIXME: this does not get triggered at all
    #[tracing::instrument]
    fn on_post_close_notif(win: &libhelium::Window) {
        let window_name = win.widget_name();
        debug!(?window_name, "Destroy event received");

        // get window id by removing the "notif-" prefix
        let window_id = window_name.split_at(6).1.parse::<u32>().unwrap();

        let tx = &NOTIF_CHANS.0;

        win.close();

        async_std::task::spawn(async move {
            tx.send(NotifStackEvent::Closed(window_id)).await.unwrap();
        });
    }

    /// Adds a [widget::Notification] into the stack directly and shows the window.
    pub fn add(&mut self, mut notif: widget::Notification, app: &libhelium::Application) {
        let id = format!("notif-{}", notif.id);
        let span = tracing::debug_span!("add_notif", id);
        let _enter = span.enter();
        debug!("Adding new notif");
        let win = notif.as_window(app, self.0.len());
        win.set_widget_name(&id);

        trace!("Setting window as visible");
        win.set_visible(true);
        let hdl_id = win.connect_destroy(Self::on_post_close_notif);
        notif.destroy_hdl_id = unsafe { hdl_id.as_raw() };
        self.0.insert(notif.id, (notif, win));
    }

    /// Checks for notifications that have timed out and removes one.
    ///
    /// Once a timed-out notification (determined by [NotifSchedTimer::is_over]) is found,
    /// the window is closed and the notification is then removed from the notification stack.
    #[tracing::instrument(skip(self))]
    fn poll(&mut self) {
        let Some((id, win, hdl_id)) = (self.0.iter())
            .find(|(_, (notif, _))| notif.sched.is_over())
            .map(|(&id, (notif, win))| (id, win, notif.destroy_hdl_id))
        else {
            return;
        };
        debug!(id, "Closing timed out notif");
        win.close();
        // FIXME: this triggers remove() twice because the destroy event fires after the window is
        // FIXME: dropped; however, without removing the window from memory, poll() will think the
        // FIXME: notif still exists in [NotificationStack].

        // workaround: remove connect_destroy() first
        win.disconnect(unsafe { glib::SignalHandlerId::from_glib(hdl_id) });
        self.remove(id);
    }

    #[tracing::instrument(skip(self))]
    pub fn remove(&mut self, index: u32) {
        debug!("Removing notif");
        self.0.remove(&index).map_or_else(
            || warn!("notif not found"),
            |(notif, win)| trace!(?notif, ?win, "notif removed"),
        );
    }

    pub fn get(&self, index: u32) -> Option<&widget::Notification> {
        self.0.get(&index).map(|obj| &obj.0)
    }
}

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
        debug!("Polling the message queue for events");
        let rx = &NOTIF_CHANS.1;

        loop {
            // we don't want CPU 100% usage
            async_std::task::sleep(std::time::Duration::from_millis(50)).await;
            if !self.stack.is_empty() {
                self.stack.poll();
            }
            let Ok(event) = rx.try_recv() else {
                if rx.is_closed() {
                    panic!("NOTIF_CHANS are closed");
                }
                continue; // rx.is_empty()
            };
            debug!(?event, "Processing event");

            match event {
                NotifStackEvent::Closed(index) => {
                    debug!(?index, "Removing notif because received close event");
                    self.stack.remove(index);
                }
                NotifStackEvent::Added(notif) => {
                    self.stack.add(notif, &self.app);
                }
            }
        }
    }
}

fn main() -> Result<gtk::glib::ExitCode> {
    // dotenvy::dotenv()?;
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("SHIZUKU_LOG").unwrap_or_else(|_| {
                println!("{NO_LOG_ENV_MSG}");
                "debug".to_string()
            }),
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
