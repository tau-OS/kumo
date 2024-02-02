use crate::dbus::Urgency;
use gtk::prelude::{BoxExt, GtkWindowExt};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use libhelium::Button;

pub struct Notification {
    pub title: String,
    pub body: String,
    pub icon: Option<String>,
    pub urgency: Urgency,
    pub id: u32,
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
        }
    }

    pub fn as_box(&self) -> gtk::Box {
        let box_ = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(10)
            .margin_top(10)
            .margin_bottom(10)
            .margin_start(10)
            .margin_end(10)
            .width_request(400)
            .height_request(100)
            .build();

        // no icon for now

        if let Some(icon) = &self.icon {
            let image = gtk::Image::builder()
                .icon_name(icon)
                .icon_size(gtk::IconSize::Large)
                .build();

            box_.append(&image);
        }

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

        let close_button = gtk::Button::builder().label("Close").build();

        action_box.append(&close_button);

        box_.append(&action_box);

        box_
    }

    pub fn as_window(&self, app: &libhelium::Application) -> libhelium::Window {
        let window = libhelium::Window::builder()
            .title(&self.title)
            .application(app)
            .resizable(false)
            .decorated(false)
            .build();

        window.init_layer_shell();

        window.set_layer(Layer::Overlay);

        window.auto_exclusive_zone_enable();

        window.set_anchor(Edge::Top, true);
        window.set_anchor(Edge::Right, true);
        window.set_anchor(Edge::Bottom, false);
        window.set_anchor(Edge::Left, false);

        window.set_margin(Edge::Top, 15);
        window.set_margin(Edge::Right, 15);

        let box_ = self.as_box();

        window.set_child(Some(&box_));

        window
    }
}
