//! Fleet is the top bar that contains app indicators, system tray, etc.
use gtk::prelude::*;
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use libhelium::prelude::*;
use relm4::prelude::*;


pub struct FleetInit {
    pub app: libhelium::Application,
    pub dbus_session: zbus::Connection,
}

kurage::generate_component!(Fleet {
    dbus_session: Option<zbus::Connection>,
}:
    preinit {
        root.set_application(Some(&init.app));
        root.init_layer_shell();
    }
    init(root, sender, model, widgets) for init: FleetInit {
        model.dbus_session = Some(init.dbus_session);
        root.auto_exclusive_zone_enable();
    }
    update(self, message, sender) {} => {}

    libhelium::ApplicationWindow {
        set_deletable: false,
        set_destroy_with_parent: true,
        set_decorated: false,
        set_resizable: true,
        set_can_focus: false,
        remove_css_class: "csd",
        set_maximized: false,
        set_exclusive_zone: 50,
        set_layer: Layer::Top,
        set_anchor[true]: Edge::Top,

        #[wrap(Some)]
        set_child = &gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_margin_top: 4,
            
            // AppTray
            // SystemTray
            // Notifications
            // Time
        },
    }
);
