///! The "bar" here is the bottom floating panel
use gtk::prelude::*;
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use libhelium::prelude::*;
use relm4::prelude::*;

pub struct BarInit {
    pub app: libhelium::Application,
    pub dbus_session: zbus::Connection,
}

kurage::generate_component!(Bar {
    dbus_session: Option<zbus::Connection>,
    app_launcher: Option<relm4::Controller<crate::components::app_launcher::AppLauncher>>,
    menu_btn: gtk::MenuButton,
}:
    preinit {
        root.set_application(Some(&init.app));
        root.init_layer_shell();
    }
    init[
        app_launcher {
            model.app_launcher = Some(
                crate::components::app_launcher::AppLauncher::builder()
                    .launch(crate::components::app_launcher::AppLauncherInit {
                        dbus_session: init.dbus_session.clone(),
                        parent: root.clone().upcast::<gtk::Widget>(),
                    })
                    .detach(),
            );
            model.app_launcher.as_ref().unwrap().widget()
        }
        menu_btn
    ](root, sender, model, widgets) for init: BarInit {
        model.dbus_session = Some(init.dbus_session);
        root.auto_exclusive_zone_enable();
    }
    update(self, message, sender) {} => {}

    libhelium::ApplicationWindow {
        // set_deletable: false,
        set_destroy_with_parent: true,
        // set_name: "Kumo Bar",
        set_decorated: false,
        set_resizable: true,
        set_can_focus: false,
        remove_css_class: "csd",
        set_maximized: false,
        set_exclusive_zone: 50,
        set_layer: Layer::Top,
        set_anchor[true]: Edge::Bottom,
        set_anchor[true]: Edge::Left,
        set_anchor[true]: Edge::Right,

        #[wrap(Some)]
        set_child = &gtk::Box {
            set_halign: gtk::Align::Start,
            set_hexpand: true,

            libhelium::Button {
                set_icon_name: "start-here",
                set_size: libhelium::ButtonSize::Small,
                set_is_iconic: false,
                set_is_pill: false,
                set_is_disclosure: true,
                set_color: libhelium::ButtonColor::Primary,
                set_valign: gtk::Align::Center,
                connect_clicked[menu_btn = model.menu_btn.clone()] => move |_| {
                    println!("Button clicked!");
                    // todo: pass in global dbus
                    // crate::util::launch_desktop("Alacritty").unwrap();
                    menu_btn.set_active(true);
                },
            },
            #[local_ref] menu_btn ->
            gtk::MenuButton {
                // note: Required to set direction to Up to force appmenu show on top
                // fixes quirks with some compositors like miriway
                set_direction: gtk::ArrowType::Up,
                set_visible: false,
                set_popover: Some(model.app_launcher.as_ref().unwrap().widget()),
            },
        },
    }
);
