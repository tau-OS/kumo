use gio::prelude::*;
use gtk::prelude::*;
use gtk4_layer_shell::{Edge, Layer, LayerShell};
// todo: Maybe use D-Bus to talk through different components? Instead of using mpsc?

const APP_ID: &str = "com.fyralabs.KiriShell";

pub struct Application {
    pub app: libhelium::Application,
}

pub fn fleet_bar(app: &libhelium::Application) -> libhelium::ApplicationWindow {
    let fleet_bar = libhelium::ApplicationWindow::builder()
        .application(app)
        .deletable(false)
        .destroy_with_parent(true)
        .name("Fleet")
        .decorated(false)
        .default_height(50)
        .resizable(true)
        .can_focus(false)
        // .opacity(0.85)
        .build();
    fleet_bar.remove_css_class("csd");
    fleet_bar.set_maximized(false);
    fleet_bar.set_exclusive_zone(50);
    fleet_bar.init_layer_shell();
    fleet_bar.set_layer(Layer::Top);
    fleet_bar.auto_exclusive_zone_enable();
    fleet_bar.set_anchor(Edge::Bottom, true);
    fleet_bar.set_anchor(Edge::Left, true);
    fleet_bar.set_anchor(Edge::Right, true);
    let container_box = {
        let b = gtk::Box::builder()
            .halign(gtk::Align::Start)
            .hexpand(true)
            .build();
        let button = libhelium::Button::builder()
            .icon_name("start-here")
            .size(libhelium::ButtonSize::Small)
            .is_iconic(false)
            .is_pill(false)
            .is_disclosure(true)
            .color(libhelium::ButtonColor::Primary)
            .valign(gtk::Align::Center)
            .build();

        button.connect_clicked(move |_| {
            println!("Button clicked!");
            crate::util::launch_desktop("Alacritty").unwrap();
        });
        b.append(&button);
        b
    };
    fleet_bar.set_child(Some(&container_box));
    fleet_bar
}

impl Application {
    pub fn new() -> Self {
        let app = libhelium::Application::new(Some(APP_ID), Default::default());

        // placeholder window
        app.connect_activate(|app| {
            // bottom bar
            let fleet_bar = fleet_bar(app);
            app.add_window(&fleet_bar);
            fleet_bar.present();

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
