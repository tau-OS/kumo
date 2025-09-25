use gtk::prelude::*;

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
            // let fleet = Fleet::new();
            // // app.add_window(&fleet);
            // fleet.set_application(Some(app));
            // fleet.present();
            // let bar = Bar::new();
            // // app.add_window(&bar);
            // bar.set_application(Some(app));
            // bar.set_child_visible(false);
            // bar.present();
        });

        Application { app }
    }

    pub fn run(&self) {
        let _hold = self.app.hold();
        self.app.run();
    }
}

// let all = gio::AppInfo::all();
// let all_appnames: Vec<String> = all
//     .iter()
//     .map(|info| info.id().unwrap_or_default().to_string())
//     .collect();
// println!("all: {:#?}", all_appnames);
