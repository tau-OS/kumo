use gtk::prelude::*;
use libhelium::prelude::*;
use relm4::prelude::*;
pub mod app_icon;
pub mod appfactory;

pub struct AppLauncherInit {
    pub parent: gtk::Widget,
}

static SEARCH_STATE: relm4::SharedState<Vec<glib::GString>> = relm4::SharedState::new();

kurage::generate_component!(AppLauncher {
    search_bar: libhelium::TextField,
    appfactory: appfactory::AppFactory,
}:
    init[search_bar appfactory { model.appfactory.0.widget() }](root, sender, model, widgets) for init: AppLauncherInit {
        // HACK: initialize app list
        *SEARCH_STATE.write() = gio::AppInfo::all().iter().map(|app| app.id().unwrap_or_default()).collect();
        search_bar.entry().connect_changed(glib::clone!(#[weak] appfactory, move |en| {
            let text = en.text();
            tracing::trace!(?text, "Input changed");
            if text.is_empty() {
                *SEARCH_STATE.write() = gio::AppInfo::all().iter().map(|app| app.id().unwrap_or_default()).collect();
            } else {
                *SEARCH_STATE.write() = gio::DesktopAppInfo::search(&*text).into_iter().flatten().collect();
            }
            appfactory.invalidate_filter();
        }));
        let appfactory2 = model.appfactory.0.clone();
        model.appfactory.set_filter_func(move |row| {
            let apps = SEARCH_STATE.read();
            if apps.is_empty() {
                // no, if search() returns nothing then nothing should be shown
                // unless the query is "" (blank string) then we just do AppInfo::all()
                tracing::debug!("No search results");
                return false
            }
            #[allow(clippy::cast_sign_loss)]
            let app_entry = appfactory2.get(row.index() as usize).unwrap();
            app_entry.deskappinfo.id().is_some_and(|id| apps.contains(&id))
        });
        // todo: probably sort by relevance...
        // model.appfactory.set_sort_func(move |row, row2| {

        // });
    }
    update(self, message, sender) {} => {}

    gtk::Popover {
        set_vexpand: true,
        set_hexpand: true,
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
            set_row_homogeneous: true,
            set_column_homogeneous: true,
            set_width_request: 1000,
            set_height_request: 300,

            // column, row, width, height
            attach[0, 0, 3, 1] = &libhelium::ViewTitle {
                set_margin_start: 4,
                set_margin_end: 4,
                set_margin_top: 4,
                set_margin_bottom: 4,
                set_label: Some("Apps"),
            },
            #[local_ref]
            attach[3, 0, 1, 1] = search_bar -> libhelium::TextField {
                set_can_focus: true,
                set_width_request: 400,
                set_hexpand: true,
                set_halign: gtk::Align::Fill,
                set_is_search: true,
                set_is_outline: true,
                set_margin_top: 10,
                set_margin_bottom: 20,
                set_margin_start: 4,
                set_margin_end: 8,
                set_prefix_icon: Some("system-search-symbolic"),
                set_placeholder_text: Some("What do you wish to do?"),
            },
            attach[0, 1, 3, 8] = &gtk::ScrolledWindow {
                set_vexpand: true,
                set_hexpand: true,
                set_min_content_width: 600,
                #[local_ref]
                appfactory -> gtk::FlowBox {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_max_children_per_line: 6,
                    set_selection_mode: gtk::SelectionMode::Single,
                    set_homogeneous: false,
                },

            },

            attach[3, 1, 1, 8] = &gtk::FlowBox {
                set_orientation: gtk::Orientation::Vertical,
                set_selection_mode: gtk::SelectionMode::Single,
                set_homogeneous: true,
                set_row_spacing: 2,
                set_column_spacing: 0,
            },
        },
    }
);
