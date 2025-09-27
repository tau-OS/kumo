use glib::clone;
use gtk::prelude::*;
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use relm4::Component;
use relm4::ComponentController;
use stable_eyre::eyre::{eyre, Result};

use crate::runtime;

// use crate::runtime;

// todo: Maybe use D-Bus to talk through different components? Instead of using mpsc?

const APP_ID: &str = "com.fyralabs.KiriShell";

pub struct Application {
    pub app: libhelium::Application,
    pub dbus_session: zbus::Connection,
}

impl Application {
    pub fn new() -> Result<Self> {
        let dbus_session = {
            let (tx, rx) = async_channel::unbounded();
            runtime().spawn(clone!(
                #[strong]
                tx,
                async move {
                    let conn = zbus::Connection::session().await.unwrap();
                    tx.send(conn).await.unwrap();
                    println!("hello")
                }
            ));
            rx.recv_blocking()?
        };
        let app = libhelium::Application::new(Some(APP_ID), Default::default());

        // placeholder window
        app.connect_activate(clone!(
            #[strong]
            dbus_session,
            move |app| {
                let mut bar = crate::components::bar::Bar::builder()
                    .launch(crate::components::bar::BarInit {
                        app: app.clone(),
                        dbus_session: dbus_session.clone(),
                    })
                    .detach();
                bar.detach_runtime();
                #[rust_analyzer::ignore]
                // RA is hallucinating an Option where there should not be one
                app.add_window(bar.widget());
                bar.widget().present();
            }
        ));

        Ok(Application { app, dbus_session })
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
