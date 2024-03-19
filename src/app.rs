use gtk::prelude::*;
use gtk4_layer_shell::{Edge, Layer, LayerShell};

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
            let fleet = Fleet::new(app.clone());
            app.add_window(&fleet.fleet);
            fleet.fleet.present();
        });

        Application { app }
    }

    pub fn run(&self) {
        self.app.run();
    }
}

// Fleet, the "panel" of the shell
pub struct Fleet {
    pub fleet: libhelium::ApplicationWindow,
}

impl Fleet {
    pub fn new(app: libhelium::Application) -> Self {
        let fleet = libhelium::ApplicationWindow::new(&app);
        fleet.set_title(Some("Fleet"));
        fleet.set_default_size(800, 40);
        fleet.init_layer_shell();
        fleet.set_layer(Layer::Top);
        fleet.auto_exclusive_zone_enable();

        let anchors = [
            (Edge::Top, true),
            (Edge::Bottom, false),
            (Edge::Left, true),
            (Edge::Right, true),
        ];

        for (edge, state) in anchors.iter() {
            fleet.set_anchor(*edge, *state);
        }

        Fleet { fleet }
    }
}
