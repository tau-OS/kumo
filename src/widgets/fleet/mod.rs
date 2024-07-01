//! Fleet is the top panel of the shell.
//! 
//! It is a panel that can be used to display the time, date, and other information.
mod imp;

use glib::subclass::types::ObjectSubclass;
use gtk::TemplateChild;
use gtk::{glib, prelude::*, subclass::prelude::*};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

glib::wrapper! {
    pub struct Fleet(ObjectSubclass<imp::Fleet>) @extends libhelium::ApplicationWindow, gtk::Window, gtk::Widget, libhelium::Window;
}

impl Default for Fleet {
    fn default() -> Self {
        Self::new()
    }
}

impl Fleet {
    pub fn new() -> Self {
        let obj: Self = glib::Object::new();
        obj.init_layer_shell();
        obj.set_layer(Layer::Top);
        obj.auto_exclusive_zone_enable();

        let anchors = [
            (Edge::Top, true),
            (Edge::Bottom, false),
            (Edge::Left, true),
            (Edge::Right, true),
        ];

        for (edge, state) in anchors.iter() {
            obj.set_anchor(*edge, *state);
        }

        obj
    }
}
