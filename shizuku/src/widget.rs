use crate::dbus::Urgency;
use glib::Cast;
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, WidgetExt};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use tracing::{debug, warn};

// top margin would be the default (window size * index)+ margin of 50
// if index is 0 then give it 15
const TOP_OFFSET: usize = 15;
const WINDOW_HEIGHT: usize = 100;

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
    pub sched: crate::NotifSchedTimer,
    pub image_data: Option<crate::icon::ImageData>,
    // pub destroy_hdl_id: u64,
}

impl Notification {
    /// Creates [libhelium::Window] for the [Notification].
    ///
    /// `index` = number of active notifications not including this new one, i.e. what the index of
    /// this notification will be once it gets added into the stack.
    pub fn as_window(&mut self, app: &libhelium::Application, index: usize) -> libhelium::Window {
        let box_ = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(10)
            .margin_top(10)
            .margin_bottom(10)
            .margin_start(0)
            .margin_end(10)
            .width_request(400)
            .height_request(WINDOW_HEIGHT as i32)
            .build();
        // force box size to be 400x100 no matter what
        box_.set_size_request(400, WINDOW_HEIGHT as i32);

        let textbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .margin_top(10)
            .spacing(10)
            .hexpand(true)
            .build();

        // map icon to None if it's an empty string
        let icon = self.icon.clone().filter(|icon| !icon.is_empty());

        let img = {
            let gimg = gtk::Image::builder()
                .icon_size(gtk::IconSize::Large)
                .margin_start(20)
                .margin_end(20)
                .pixel_size(50)
                .css_classes(vec!["circle-radius"])
                .halign(gtk::Align::Center)
                .valign(gtk::Align::Center)
                .vexpand(false)
                .name("icon")
                .build();

            if let Some(imgdata) = &self.image_data {
                gimg.set_from_pixbuf(Some(&imgdata.into()));
            } else if let Some(icon) = &icon {
                // check if icon is a file path or a generic icon name
                if std::path::PathBuf::from(icon).is_file() {
                    gimg.set_from_file(Some(icon));
                } else {
                    gimg.set_from_icon_name(Some(icon));
                }
            } else {
                gimg.set_margin_end(0);
                gimg.set_margin_start(0);
            }
            gimg
        };

        box_.append(&img);

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
            .use_markup(true)
            .halign(gtk::Align::Start)
            .lines(3)
            .max_width_chars(30)
            .wrap(true)
            .wrap_mode(gtk::pango::WrapMode::WordChar)
            // .width_request(300)
            .ellipsize(gtk::pango::EllipsizeMode::End)
            .build();

        // We add markup to the body too, conforming to XDG spec
        body.set_markup(&self.body);

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

            debug!(?window_name, ?window_id, "Clicked close button");

            // WORKAROUND: Let's send a message to close the window

            // FIXME: So main::NotificationStack::on_post_close_notif is not getting called at all
            // unless we emit this message to the queue
            // which makes this closure redundant

            let tx = &crate::NOTIF_CHANS.0;

            glib::MainContext::default().spawn_local(async move {
                tx.send(crate::NotifStackEvent::Closed(window_id))
                    .await
                    .unwrap();
            });

            window.close();
        });

        action_box.append(&close_button);

        box_.append(&action_box);

        let window = libhelium::Window::builder()
            .title(&self.title)
            .application(app)
            .resizable(false)
            .decorated(false)
            // Set opacity to be barely transparent, works around https://github.com/WayfireWM/wayfire/issues/2125
            // NOTE: Only 2 decimal places work, 3 or more will round up to 1.0, thus making it opaque again
            .opacity(0.99)
            .css_classes(vec!["surface-container-lowest-bg-color", "x-large-radius"])
            .css_name("notif-toast")
            .build();

        // force window size to be 400x100 no matter what

        window.set_size_request(400, WINDOW_HEIGHT as i32);

        // FIXME: wrong placaement when manually close notif then make more (will duplicate at same
        // position due to using index of stack (which has new shorter length))
        let top_margin = (WINDOW_HEIGHT + 50) * index + TOP_OFFSET;

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

        window.set_child(Some(&box_));

        // let mut windows = GTK_WINDOWS.with(|windows| windows.lock().unwrap().clone());

        // windows.push(window.clone());

        window
    }
}
