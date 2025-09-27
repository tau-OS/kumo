use gtk::prelude::*;
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use libhelium::prelude::*;
use relm4::prelude::*;
pub mod app_icon;

pub struct AppLauncherInit {
    pub dbus_session: zbus::Connection,
    pub parent: gtk::Widget,
}

kurage::generate_component!(AppLauncher {
    dbus_session: Option<zbus::Connection>,
}:
    init(root, sender, model, widgets) for init: AppLauncherInit {
        model.dbus_session = Some(init.dbus_session);
        // root.set_relative_to(Some(init.parent));
    }
    update(self, message, sender) {} => {}

    gtk::Popover {
        #[wrap(Some)]
        set_child = &gtk::Grid {
            set_margin_all: 4,

            set_column_spacing: 8,
            set_row_spacing: 4,
            
            // top, left, width, height
            attach[0, 0, 12, 1] = &gtk::Label {
                set_text: "Apps",
            },
            attach[0, 13, 8, 1] = &libhelium::TextField {
                set_is_search: true,
                set_placeholder_text: Some("What do you wish to do?"),
            },
            attach[1, 0, 12, 10] = &gtk::FlowBox {
                set_orientation: gtk::Orientation::Vertical,
                set_max_children_per_line: 1,
                set_selection_mode: gtk::SelectionMode::Single,
            },
            attach[1, 13, 8, 10] = &gtk::FlowBox {
                set_orientation: gtk::Orientation::Vertical,
                set_selection_mode: gtk::SelectionMode::Single,
            },
        },
    }
);
