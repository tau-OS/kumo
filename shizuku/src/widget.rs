use crate::dbus::Urgency;
use glib::Cast;
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, WidgetExt};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use tracing::{debug, warn};
const WINDOW_HEIGHT: i32 = 100;

// thread_local! {
//     pub static GTK_WINDOWS: std::sync::Arc<std::sync::Mutex<Vec<libhelium::Window>>> = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
// }

// pub fn find_window_by_name(name: &str) -> Option<libhelium::Window> {
//     let windows = GTK_WINDOWS.with(|windows| windows.lock().unwrap().clone());

//     windows
//         .iter()
//         .find(|window| window.widget_name() == name)
//         .cloned()
// }

// pub fn delete_window_by_name(name: &str) {
//     debug!(?name, "Deleting window by name");
//     let mut windows = GTK_WINDOWS.with(|windows| windows.lock().unwrap().clone());

//     let window_idx = windows
//         .iter()
//         .position(|window| window.widget_name() == name);

//     if let Some(idx) = window_idx {
//         let window = windows.get(idx).unwrap();

//         window.close();
//         windows.remove(idx);
//     }

// }

#[derive(Default, Clone, Debug)]
pub struct Notification {
    pub title: String,
    pub body: String,
    pub icon: Option<String>,
    pub urgency: Urgency,
    pub id: u32,
    // we need this to set the connect() stuff in the NotificationStack
    // pub close_btn: Option<Button>,
    pub sched: Option<crate::NotifSchedTimer>,
}

impl Notification {
    pub fn new(
        title: String,
        body: String,
        icon: Option<String>,
        urgency: Urgency,
        id: u32,
    ) -> Self {
        Self {
            title,
            body,
            icon,
            urgency,
            id,
            // close_btn: None,
            sched: None,
        }
    }

    pub fn as_window(&mut self, app: &libhelium::Application, index: usize) -> libhelium::Window {
        let box_ = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(10)
            .margin_top(10)
            .margin_bottom(10)
            .margin_start(10)
            .margin_end(10)
            .width_request(400)
            .height_request(WINDOW_HEIGHT)
            .build();

        // no icon for now

        let image = &gtk::Image::builder()
            .icon_size(gtk::IconSize::Large)
            // .size_request(50, 50)
            .build();
        self.icon.as_ref().map(|icon| {
            image.set_from_icon_name(Some(icon));
            image.set_size_request(60, 50);
            box_.append(image);
        });

        let textbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .margin_top(5)
            .spacing(10)
            .hexpand(true)
            .build();

        let title = gtk::Label::builder()
            .label(&self.title)
            .use_markup(true)
            .name("title")
            // .set_markup(&format!("<b>{}</b>", &self.title))
            .halign(gtk::Align::Start)
            .lines(1)
            .ellipsize(gtk::pango::EllipsizeMode::End)
            .css_classes(vec!["bold"])
            .build();

        title.set_markup(&format!("<b>{}</b>", &self.title));

        let body = gtk::Label::builder()
            .label(&self.body)
            .halign(gtk::Align::Start)
            .lines(3)
            .ellipsize(gtk::pango::EllipsizeMode::End)
            .build();

        textbox.append(&title);
        textbox.append(&body);

        box_.append(&textbox);

        // Action button with close box

        let action_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(10)
            .halign(gtk::Align::End)
            .valign(gtk::Align::Start)
            .build();

        let close_button = gtk::Button::builder()
            .icon_name("window-close")
            .css_classes(vec!["close-button", "circle-radius"])
            // .css_name("notif-close-btn")
            .build();

        close_button.connect_clicked(move |btn| {
            let Some(window) = btn
                .parent()
                .and_then(|action| action.parent())
                .and_then(|box_| box_.parent())
            else {
                warn!("Can't get parent in connect clicked");
                return;
            };

            // downcast to libhelium::Window
            let window = window
                .downcast::<libhelium::Window>()
                .expect("fail to downcast");

            let window_name = window.widget_name();
            // parse window id by removing the "notif-" prefix
            let window_id = window_name.split_at(6).1.parse::<u32>().unwrap();

            debug!(?window_name, ?window_id, "Notif window closed");

            window.close();
        });

        action_box.append(&close_button);
        // self.close_btn = Some(close_button);

        box_.append(&action_box);

        let window = libhelium::Window::builder()
            .title(&self.title)
            .application(app)
            .resizable(false)
            .decorated(false)
            .css_classes(vec!["surface-container-lowest-bg-color", "x-large-radius"])
            .css_name("notif-toast")
            .build();

        // top margin would be the default (window size * index)+ margin of 50
        // if index is 0 then give it 15

        let top_offset = 15;

        let top_margin = if index == 0 {
            0 + top_offset
        } else {
            ((index * WINDOW_HEIGHT as usize) + 50 * index) + top_offset
        };

        debug!(?top_margin);

        window.init_layer_shell();

        window.set_layer(Layer::Overlay);

        window.connect_show(move |window| {
            window.auto_exclusive_zone_enable();

            window.set_anchor(Edge::Top, true);
            window.set_anchor(Edge::Right, true);
            window.set_anchor(Edge::Bottom, false);
            window.set_anchor(Edge::Left, false);

            window.set_margin(Edge::Top, top_margin as i32 + 15);
            window.set_margin(Edge::Right, 15);
            window.present();
        });

        // let box_ = self.as_box();

        window.set_child(Some(&box_));

        // let mut windows = GTK_WINDOWS.with(|windows| windows.lock().unwrap().clone());

        // windows.push(window.clone());

        window

        // box_
    }

    /// Keep polling until the current time is greater than the scheduled time,
    /// then, destroy the window
    pub async fn poll(&mut self) -> bool {
        let sched = self.sched.as_mut();

        if let Some(sched) = sched {
            if sched.is_over() {
                debug!("Sched is over");
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}
