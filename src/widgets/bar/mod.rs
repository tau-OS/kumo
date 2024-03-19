//! KIRI Bar widget.
//! The Bar is the bottom "dock" of the desktop, where applications can be pinned and launched.
mod imp;

use glib::subclass::types::ObjectSubclass;
use gtk::TemplateChild;
use gtk::{glib, prelude::*, subclass::prelude::*};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

glib::wrapper! {
    pub struct Bar(ObjectSubclass<imp::Bar>) @extends libhelium::ApplicationWindow, gtk::Window, gtk::Widget, libhelium::Window;
}

impl Bar {
    pub fn new() -> Self {
        let obj: Self = glib::Object::new();
        obj.init_layer_shell();
        obj.set_layer(Layer::Top);
        // obj.auto_exclusive_zone_enable();

        let anchors = [
            (Edge::Top, false),
            (Edge::Bottom, true),
            (Edge::Left, false),
            (Edge::Right, false),
        ];

        for (edge, state) in anchors.iter() {
            obj.set_anchor(*edge, *state);
        }

        obj
    }
}

glib::wrapper! {
    pub struct AppIcon(ObjectSubclass<imp::AppIcon>) @extends gtk::Button, gtk::Widget;
}

impl AppIcon {
    pub fn new() -> Self {
        let obj = glib::Object::new();

        obj
    }
}
