use relm4::prelude::*;
use gtk::prelude::*;

kurage::generate_component!(Notif:
    init(root, sender, model, widgets) {
    }
    update(self, message, _sender) {
    } => {}

    gtk::MenuButton {
        // set_icon_name: "notifications-symbolic",
        // set_icon_name: "notification-new-symbolic",
        set_icon_name: "message-symbolic", // message-new-symbolic
        set_direction: gtk::ArrowType::Up,
        inline_css: ".image-button { padding: 0; }",
        
        #[wrap(Some)]
        set_popover = &gtk::Popover {
            set_position: gtk::PositionType::Bottom,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_all: 12,
                set_spacing: 12,
                
                gtk::Label {
                    set_label: "Test Notification",
                    set_margin_all: 12,
                },
            },
        },
    },
);
