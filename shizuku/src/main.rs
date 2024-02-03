mod dbus;
mod widget;
use color_eyre::Result;
use gio::{
    glib::{clone, Cast},
    prelude::{ApplicationExt, ApplicationExtManual},
};
use gtk::prelude::{ButtonExt, GtkWindowExt, WidgetExt};
use std::{
    sync::{mpsc, Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};
use tracing::{debug, warn};

lazy_static::lazy_static! {
    static ref NOTIF_DESTROY_CHANS: std::sync::Arc<(async_std::channel::Sender<NotifStackEvent>, async_std::channel::Receiver<NotifStackEvent>)>
        = std::sync::Arc::new(async_std::channel::unbounded());
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

    pub fn is_over(&self) -> bool {
        time_now() >= self.until
    }
}

#[derive(Debug, Clone)]
pub enum NotifStackEvent {
    Closed(usize), // pos of notif in the stack vec
    Added(widget::Notification),
}
#[derive(Clone)]
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

    #[tracing::instrument]
    fn on_post_close_notif(win: &libhelium::Window) {
        let window_name = win.widget_name();
        debug!(?window_name, "Notif window closed");

        win.close();

        // emit signal to close the notif
        // todo!()
    }

    pub fn add(&mut self, mut notif: widget::Notification, app: &libhelium::Application) {
        let win = notif.as_window(app, self.0.len());
        win.set_widget_name(&format!("notif-{}", notif.id));
        // (notif.close_btn.as_ref())
        // .expect("No close button registered on notif")
        // .connect_clicked(Self::on_notif_close_btn_clicked);

        win.set_visible(true);
        win.connect_destroy(Self::on_post_close_notif);
        notif.sched = Some(NotifSchedTimer::new());
        self.0.push(notif);
    }

    // todo: recieve signals from NOTIF_DESTROY_CHANS.1 instead of using this method
    pub fn poll(&mut self) {
        println!("Polling notifs");

        let MSGS = NOTIF_DESTROY_CHANS.clone();

        let (tx, rx) = (MSGS.0.clone(), MSGS.1.clone());

        let mut notifs_to_rm = vec![];
        for (i, notif) in self.0.iter().enumerate() {
            if notif.sched.as_ref().expect("No notif sched").is_over() {
                // pretend we're closing it
                // Self::on_notif_close_btn_clicked(&notif.close_btn.as_ref().expect("No close btn"));
                notifs_to_rm.push(i);
            }
        }
        (notifs_to_rm.into_iter().enumerate()).for_each(|(n, i)| self.post_close_notif(i - n));
    }

    #[tracing::instrument(skip(self))]
    pub fn remove(&mut self, index: usize) {
        // find notification with ID == index and remove it
        let a = self
            .0
            .iter()
            .position(|notif| notif.id == index.try_into().unwrap());

        if let Some(pos) = a {
            self.0.remove(pos);
        }
    }

    pub fn get(&self, index: usize) -> Option<&widget::Notification> {
        self.0.get(index)
    }

    // pub async fn msg_poll(&mut self) -> Result<()> {
    //     debug!("Polling for notif events");
    //     let channel = NOTIF_DESTROY_CHANS.clone();

    //     let (_tx, rx) = (channel.0.clone(), channel.1.clone());

    //     loop {
    //         let event = rx.recv().await?;
    //         event.process(self);
    //     }
    // }
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

    pub fn add(&mut self, notif: widget::Notification) {
        self.stack.add(notif, &self.app);
    }

    pub fn run(&mut self) -> gtk::glib::ExitCode {
        // self.app.connect_activate(move |app| {
        //     let msgs = NOTIF_DESTROY_CHANS.clone();

        //     let (tx, rx) = (msgs.0.clone(), msgs.1.clone());

        //     gtk::glib::MainContext::default().spawn_local(clone!(@strong app => async move {
        //         loop {
        //             let event = rx.recv().await.unwrap();

        //             match event {
        //                 NotifStackEvent::Closed(index) => {
        //                     self.stack.lock().unwrap().remove(index);
        //                 },
        //                 NotifStackEvent::Added(notif) => {
        //                     self.add(notif);
        //                 }
        //             }

        //         }
        //     }));
        // });

        let mut self_clone = self.clone();
        gtk::glib::MainContext::default().spawn_local(async move {
            self_clone.poll_msg_queue().await;
        });
        let _ = self.app.hold();
        self.app.run()
    }

    pub async fn poll_msg_queue(&mut self) {
        debug!("Polling for notif events");
        let msgs = NOTIF_DESTROY_CHANS.clone();

        let rx = msgs.1.clone();

        loop {
            let event = rx.recv().await.unwrap();

            match event {
                NotifStackEvent::Closed(index) => {
                    self.stack.remove(index);
                }
                NotifStackEvent::Added(notif) => {
                    self.add(notif);
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

    async_std::task::spawn(clone!(@strong application => async move {
        // application.poll_msg_queue().await;
    }));

    async_std::task::spawn(async {
        tracing::info!("Starting dbus server");
        let connection = zbus::Connection::session().await.unwrap();
        connection
            .object_server()
            .at(dbus::DBUS_OBJECT_PATH, dbus::NotificationsServer)
            .await
            .unwrap();

        connection.request_name(dbus::DBUS_INTERFACE).await.unwrap();

        loop {
            std::future::pending::<()>().await;
        }
    });

    // let application = libhelium::Application::builder()
    // .application_id("com.fyralabs.shizuku")
    // .build();

    // todo: bind this code to an actual dbus server,
    // also, make them windows stackable so we can have multiple notifications
    application.app.connect_activate(|app| {
        let mut stack = NotificationStack::new(vec![]);
        // stack.add(
        //     widget::Notification::new(
        //         "Hello".to_string(),
        //         "This is a notification".to_string(),
        //         None,
        //         dbus::Urgency::Low,
        //         0,
        //     ),
        //     app,
        // );

        // // std::thread::sleep(std::time::Duration::from_secs(5));
        // stack.add(
        //     widget::Notification::new(
        //         "Hello".to_string(),
        //         "This is a notification too".to_string(),
        //         None,
        //         dbus::Urgency::Low,
        //         0,
        //     ),
        //     app,
        // );

        // gtk::glib::spawn_future_local(async {
        //     stack.
        // });

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

    // We hold the application here so that it doesn't exit immediately after the activate signal is emitted
    let _hold = application.app.hold();

    // Ok(application.app.run())
    Ok(application.run())
}
