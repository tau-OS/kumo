//! Fleet is the top bar that contains app indicators, system tray, etc.
mod datetime;
mod notif;

use gtk::prelude::*;
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use libhelium::prelude::*;
use relm4::prelude::*;

pub struct FleetInit {
    pub app: libhelium::Application,
}

macro_rules! gen_ctrls {
    ($FleetCtrls:ident $($comp:ident)+) => { ::pastey::paste! {
        #[derive(Debug)]
        struct $FleetCtrls { $(
            [<$comp _comp>]: relm4::Controller<$comp::[<$comp:camel>]>,
        )+}
        impl Default for $FleetCtrls {
            fn default() -> Self {
                Self { $(
                    [<$comp _comp>]: $comp::[<$comp:camel>]::builder().launch(()).detach(),
                )+}
            }
        }
    }};
}

gen_ctrls!(FleetCtrls datetime notif);

kurage::generate_component!(Fleet {
    ctrls: FleetCtrls,
    ctrlbox: gtk::Box,
}:
    preinit {
        root.set_application(Some(&init.app));
        root.init_layer_shell();
    }
    init[ctrlbox](root, sender, model, widgets) for init: FleetInit {
        root.auto_exclusive_zone_enable();
    }
    update(self, message, _sender) {} => {}

    libhelium::ApplicationWindow {
        set_deletable: false,
        set_destroy_with_parent: true,
        set_decorated: false,
        set_resizable: false,
        set_can_focus: false,
        remove_css_class: "csd",
        remove_css_class: "background",
        set_maximized: false,
        set_exclusive_zone: 50,
        set_layer: Layer::Top,
        set_anchor[true]: Edge::Top,
        set_anchor[true]: Edge::Right,
        inline_css: "background: rgba(0,0,0,0.5); border-bottom-left-radius: 10px; padding: 4px;",
        
        // connect_show: |win| {
        //     win.child().unwrap().set_margin_end(8);
        // },

        #[local_ref]
        #[wrap(Some)]
        set_child = ctrlbox -> gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_hexpand: true,
            set_vexpand: true,
            set_spacing: 16,
            set_margin_start: 8,
            set_margin_end: 12,
            
            prepend = model.ctrls.datetime_comp.widget(),
            prepend = model.ctrls.notif_comp.widget(),
        },
    }
);
