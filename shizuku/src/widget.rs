use crate::dbus::Urgency;
use gio::glib::ObjectExt;
use gtk::{
    gdk::ffi::{gdk_display_get_default_seat, GdkDisplay},
    prelude::{BoxExt, ButtonExt, GtkWindowExt, WidgetExt},
    Button,
};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use tracing::{debug, warn};

const WINDOW_HEIGHT: i32 = 100;

#[derive(Default, Clone)]
pub struct Notification {
    pub title: String,
    pub body: String,
    pub icon: Option<String>,
    pub urgency: Urgency,
    pub id: u32,
    // we need this to set the connect() stuff in the NotificationStack
    pub close_btn: Option<Button>,
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
            close_btn: None,
            sched: None,
        }
    }

    pub fn as_box(&mut self) -> gtk::Box {
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

        self.icon.as_ref().map(|icon| {
            box_.append(
                &gtk::Image::builder()
                    .icon_name(icon)
                    .icon_size(gtk::IconSize::Large)
                    .build(),
            )
        });

        let textbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(10)
            .hexpand(true)
            .build();

        let title = gtk::Label::builder()
            .label(&self.title)
            .halign(gtk::Align::Start)
            .lines(1)
            .ellipsize(gtk::pango::EllipsizeMode::End)
            .build();

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
            .css_classes(vec!["close-button", "rounded"])
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
            window.set_visible(false);
        });

        action_box.append(&close_button);
        self.close_btn = Some(close_button);

        box_.append(&action_box);

        box_
    }

    pub fn as_window(&mut self, app: &libhelium::Application, index: usize) -> libhelium::Window {
        let window = libhelium::Window::builder()
            .title(&self.title)
            .application(app)
            .resizable(false)
            .decorated(false)
            .build();

        // top margin would be the default (window size * index)+ margin of 50
        // if index is 0 then give it 15

        let top_margin = if index == 0 {
            15
        } else {
            (index * WINDOW_HEIGHT as usize) + 50
        };

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

        let box_ = self.as_box();

        window.set_child(Some(&box_));

        // box_.connect("motion-notify-event", false, |_| {
        //     debug!("motion notify!");
        //     None
        // });

        // on activate, show window for 5 seconds and then close it

        /*    window.connect_visible_notify(|window| {
            println!("Window activated");
            // window.set_visible(true);
            window.show();

            // wait for 5 seconds and then close the window

            std::thread::sleep(std::time::Duration::from_secs(5));

            window.set_visible(false);
        }); */

        // self.window = Some(window);

        window
    }
}
