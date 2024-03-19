use std::time::Duration;

use crate::widgets::bar::Bar;
use crate::widgets::fleet::Fleet;
use gtk::prelude::*;
use std::sync::{Mutex, RwLock};

// todo: Maybe use D-Bus to talk through different components? Instead of using mpsc?

const APP_ID: &str = "com.fyralabs.KiriShell";

pub struct Application {
    pub app: libhelium::Application,
}

impl Application {
    pub fn new() -> Self {
        let app = libhelium::Application::new(Some(APP_ID), Default::default());

        // placeholder window
        app.connect_activate(|app| {
            let fleet = Fleet::new();
            // app.add_window(&fleet);
            fleet.set_application(Some(app));
            fleet.present();

            // todo: glib::timeout_add for ticking clock
            /*
            glib::timeout_add(Duration::from_millis(500), move || {
                // fleet.tick_clock();

                glib::clone!(@weak fleet => move || {
                    fleet.tick_clock();
                });
                glib::ControlFlow::Continue
            }); */

            let bar = Bar::new();
            // app.add_window(&bar);
            bar.set_application(Some(app));
            bar.set_child_visible(false);
            bar.present();

            // Use fleet_box, fleet_label, fleet_window in the code
        });

        Application { app }
    }

    pub fn run(&self) {
        self.app.run();
    }
}
