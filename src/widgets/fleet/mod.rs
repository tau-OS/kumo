mod imp;

use glib::subclass::types::ObjectSubclass;
use gtk::TemplateChild;
use gtk::{glib, prelude::*, subclass::prelude::*};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

glib::wrapper! {
    pub struct Fleet(ObjectSubclass<imp::Fleet>) @extends libhelium::ApplicationWindow, gtk::Window, gtk::Widget, libhelium::Window;
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

        // todo: replace with Clock widget
        obj.connect_realize(|fleet| {
            let args = fleet.clone();
            glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
                args.tick_clock();
                glib::ControlFlow::Continue
            });
        });

        obj
    }

    pub fn tick_clock(&self) {
        // imp::Fleet::on_clock_tick(self::imp::Fleet::from_obj(self));
    }
}
