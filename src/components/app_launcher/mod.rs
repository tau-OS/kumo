use gtk::prelude::*;
use libhelium::prelude::*;
use relm4::prelude::*;
pub mod app_icon;
pub mod appfactory;

pub struct AppLauncherInit {
    pub dbus_session: zbus::Connection,
    pub parent: gtk::Widget,
}

static SEARCH_STATE: relm4::SharedState<glib::GString> = relm4::SharedState::new();

kurage::generate_component!(AppLauncher {
    dbus_session: Option<zbus::Connection>,
    search_bar: libhelium::TextField,
    appfactory: appfactory::AppFactory,
}:
    init[search_bar appfactory { model.appfactory.0.widget() }](root, sender, model, widgets) for init: AppLauncherInit {
        search_bar.internal_entry().connect_changed(glib::clone!(#[weak] appfactory, move |en| {
            let text = en.text();
            tracing::trace!(?text, "Input changed");
            *SEARCH_STATE.write() = text;
            appfactory.invalidate_filter();
        }));
        // model.dbus_session = Some(init.dbus_session);
        let appfactory2 = model.appfactory.0.clone();
        model.appfactory.set_filter_func(move |row| {
            let s = SEARCH_STATE.read().as_str().to_ascii_lowercase();
            #[allow(clippy::cast_sign_loss)]
            let app_entry = appfactory2.get(row.index() as usize).unwrap();
            app_entry.name.to_ascii_lowercase().starts_with(&s) || app_entry.keywords.iter().any(|keyword| keyword.to_ascii_lowercase().starts_with(&s))
        });
    }
    update(self, message, sender) {} => {}

    gtk::Popover {
        // set_vexpand: true,
        // set_hexpand: true,
        // todo: disable below, this one is for testing
        set_autohide: false,

        set_default_widget: Some(&model.search_bar),

        #[wrap(Some)]
        set_child = &gtk::Grid {
            set_margin_all: 4,
            set_vexpand: true,
            set_hexpand: true,
            set_row_spacing: 4,
            set_column_spacing: 4,
            // set_row_homogeneous: true,
            // set_column_homogeneous: true,

            // column, row, width, height
            attach[0, 0, 12, 1] = &libhelium::ViewTitle {
                set_label: Some("Apps"),
            },
            #[local_ref]
            attach[13, 0, 8, 1] = search_bar -> libhelium::TextField {
                set_hexpand: true,
                set_halign: gtk::Align::Fill,
                set_is_search: true,
                set_is_outline: true,
                set_margin_top: 6,
                set_margin_bottom: 6,
                set_prefix_icon: Some("system-search-symbolic"),
                set_placeholder_text: Some("What do you wish to do?"),
            },
            #[local_ref]
            attach[0, 1, 12, 10] = appfactory -> gtk::FlowBox {
                set_orientation: gtk::Orientation::Horizontal,
                set_max_children_per_line: 6,
                set_selection_mode: gtk::SelectionMode::Single,
                set_homogeneous: true,
            },
            attach[13, 1, 8, 10] = &gtk::FlowBox {
                set_orientation: gtk::Orientation::Vertical,
                set_selection_mode: gtk::SelectionMode::Single,
            },
        },
    }
);
