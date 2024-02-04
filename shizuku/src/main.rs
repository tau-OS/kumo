mod dbus;
mod widget;
use color_eyre::Result;
use gio::prelude::ApplicationExtManual;
use gtk::prelude::{GtkWindowExt, WidgetExt};
use std::{
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};
use tracing::{debug, trace, warn};

lazy_static::lazy_static! {
    static ref NOTIF_DESTROY_CHANS: std::sync::Arc<(async_std::channel::Sender<NotifStackEvent>, async_std::channel::Receiver<NotifStackEvent>)>
        = std::sync::Arc::new(async_std::channel::unbounded());
}

thread_local! {
    pub static NOTIFICATION_STACK: Arc<Mutex<NotificationStack>> = Arc::new(Mutex::new(NotificationStack::new(vec![])));
}

fn time_now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
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

// todo: @madonuko, make this a hashmap of (notif, gtk::Window) so we can access windows directly?
// helps for remotely and automatically closing notifs
#[derive(Clone)]
pub struct NotificationStack(Vec<widget::Notification>);

impl NotificationStack {
    pub fn new(notifications: Vec<widget::Notification>) -> Self {
        Self { 0: notifications }
    }

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

        // emit signal to close the notif
        // todo!()
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
        self.0.push(notif);
        // Now create a new thread to poll timers and close notifs

        // async_std::task::spawn(clone!(@weak win => async move {
        //     let mut sched = notif.sched.as_mut().expect("No notif sched");
        //     while !sched.is_over() {
        //         async_std::task::sleep(std::time::Duration::from_secs(1)).await;
        //     }
        //     // Self::on_post_close_notif(win.as_ref().expect("No window"));

        //     win.close();
        // }));
    }

    #[tracing::instrument(skip(self))]
    pub fn poll(&mut self) -> Option<NotifStackEvent> {
        self.0
            .iter()
            .find(|notif| notif.sched.as_ref().expect("No notif sched").is_over())
            .map(|notif| {
                debug!(id = notif.id, "Found notif that should be closed!");
                NotifStackEvent::Closed(notif.id)
            })
    }

    #[tracing::instrument(skip(self))]
    pub fn remove(&mut self, index: u32) {
        // find notification with ID == index and remove it
        if let Some(pos) = self.0.iter().position(|notif| notif.id == index) {
            debug!(pos, "Removing notif");
            self.0.remove(pos);
        }
    }

    pub fn get(&self, index: usize) -> Option<&widget::Notification> {
        self.0.get(index)
    }
}

const APPLICATION_ID: &str = "com.fyralabs.shizuku";

#[derive(Clone)]
pub struct Application {
    pub app: libhelium::Application,
    pub stack: NotificationStack,
}

impl Application {
    pub fn new() -> Self {
        let app = libhelium::Application::builder()
            .application_id(APPLICATION_ID)
            .flags(gio::ApplicationFlags::NON_UNIQUE)
            .build();

        let stack = NotificationStack::new(vec![]);

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
        let msgs = NOTIF_DESTROY_CHANS.clone();

        let rx = msgs.1.clone();

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
        }
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

    let mut application = Application::new();

    // async_std::task::spawn(clone!(@strong application => async move {
    // application.poll_msg_queue().await;
    // }));

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

    // Ok(application.app.run())
    Ok(application.run())
}
