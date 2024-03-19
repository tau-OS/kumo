use glib::subclass::types::ObjectSubclass;
use gtk::{glib, prelude::*, subclass::prelude::*};
use gtk::{CompositeTemplate, TemplateChild};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use crate::widgets::fleet::Fleet;

// todo: Maybe use D-Bus to talk through different components? Instead of using mpsc?

const APP_ID: &str = "com.fyralabs.KiriShell";

pub struct Application {
    pub app: libhelium::Application,
}

impl Application {
    pub fn new() -> Self {
        let mut app = libhelium::Application::new(Some(APP_ID), Default::default());

        // placeholder window
        app.connect_activate(|app| {
            /*             let fleet = Fleet::new(app.clone());
            app.add_window(&fleet.fleet);
            fleet.fleet.present(); */
            let fleet = Fleet::new();
            app.add_window(&fleet);
            fleet.set_application(Some(app));
            //     ^ ❓???? doesn't satisfy `fleet::Fleet: IsA<gtk4::Window>`
            fleet.present();
        });

        Application { app }
    }

    pub fn run(&self) {
        self.app.run();
    }
}
