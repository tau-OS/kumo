use std::ops::ControlFlow;

use crate::dbus::Urgency;
use gio::glib::{clone::Downgrade, Cast};
use glib::ControlFlow::Continue;
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, WidgetExt};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use libhelium::Button;

const APP_ID: &str = "com.fyralabs.shizuku";

pub struct NotificationStack {
    pub notifications: Vec<Notification>,
}

impl NotificationStack {
    pub fn new(notifications: Vec<Notification>) -> Self {
        Self { notifications }
    }

    pub fn clear(&mut self) {
        self.notifications.clear();
    }

    pub fn add(&mut self, notif: Notification, app: &libhelium::Application) {
        self.notifications.push(notif);

        // then get the count of all notifications
        let count = self.notifications.len();

        // clamp at 0
        let count = if count == 1 { 0 } else { count - 1 };
        // now show with index
        self.notifications
            .last_mut()
            .unwrap()
            .as_window(app, count)
            .set_visible(true);
    }
}

pub struct Application {
    pub app: libhelium::Application,
}

impl Application {
    pub fn new() -> Self {
        let app = libhelium::Application::builder()
            .application_id(APP_ID)
            .flags(gio::ApplicationFlags::NON_UNIQUE)
            .build();
        Self { app }
    }
}
const WINDOW_HEIGHT: i32 = 100;

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
        window.set_namespace("notification");
        window.auto_exclusive_zone_enable();
        window.connect_destroy_with_parent_notify(|window| {
            println!("Window destroyed");
            window.set_visible(false);
        });

        window.set_layer(Layer::Top);
        window.add_css_class("notification");
        window.set_default_size(400, WINDOW_HEIGHT);

        window.connect_show(move |window| {
            let cloned_window = window.clone();
            window.auto_exclusive_zone_enable();

            window.set_anchor(Edge::Top, true);
            window.set_anchor(Edge::Right, true);
            window.set_anchor(Edge::Bottom, false);
            window.set_anchor(Edge::Left, false);

            window.set_margin(Edge::Top, top_margin as i32 + 15);
            window.set_margin(Edge::Right, 15);
            window.present();
            gtk::glib::spawn_future_local(async move {
                // wait 5 seconds and then close the window
                // std::thread::sleep(std::time::Duration::from_secs(5));

                // cloned_window.close();
            });
            window.present();
        });

        let box_ = {
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

            let close_button = gtk::Button::builder()
                .icon_name("window-close")
                .css_classes(vec!["close-button", "rounded"])
                .build();

            close_button.connect_clicked(|_button| {
                // window.destroy();
                println!("Close button clicked");
                // send signal to close window
                // window.activate();
            });

            action_box.append(&close_button);

            box_.append(&action_box);

            box_
        };

        &window.set_child(Some(&box_));

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

        let w = window.downgrade();

        window
    }
}
