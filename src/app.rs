use glib::clone;
use gtk::prelude::*;
use relm4::Component;
use relm4::ComponentController;
use stable_eyre::eyre::{eyre, Result};
use std::sync::OnceLock;
use zbus::Connection;

use crate::runtime;

pub static DBUS_SESSION: OnceLock<zbus::Connection> = OnceLock::new();

/// Initialize the D-Bus session connection asynchronously
pub(crate) fn init_dbus() {
    // tokio spawn() does not need to be awaited,
    // it should resolve on its own
    runtime().spawn(async {
        DBUS_SESSION
            .set(zbus::Connection::session().await.unwrap())
            .unwrap();
        tracing::info!("Initialized D-Bus session");
    });
}

// use crate::runtime;

// todo: Maybe use D-Bus to talk through different components? Instead of using mpsc?

const APP_ID: &str = "com.fyralabs.KiriShell";

pub struct Application {
    pub app: libhelium::Application,
}

impl Application {
    pub fn new() -> Result<Self> {
        let app = libhelium::Application::new(Some(APP_ID), Default::default());

        // placeholder window
        app.connect_activate(clone!(move |app| {
            let mut bar = crate::components::bar::Bar::builder()
                .launch(crate::components::bar::BarInit { app: app.clone() })
                .detach();
            bar.detach_runtime();
            #[rust_analyzer::ignore]
            // RA is hallucinating an Option where there should not be one
            app.add_window(bar.widget());
            bar.widget().present();
        }));

        Ok(Application { app })
    }

    pub fn run(&self) {
        // let _hold = self.app.hold();
        self.app.run();
        ()
    }
}

// let all = gio::AppInfo::all();
// let all_appnames: Vec<String> = all
//     .iter()
//     .map(|info| info.id().unwrap_or_default().to_string())
//     .collect();
// println!("all: {:#?}", all_appnames);
