use glib::subclass::types::ObjectSubclass;
use gtk::{glib, prelude::*, subclass::prelude::*};
use gtk::{CompositeTemplate, TemplateChild};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

// todo: Maybe use D-Bus to talk through different components? Instead of using mpsc?

const APP_ID: &str = "com.fyralabs.KiriShell";

pub struct Application {
    pub app: libhelium::Application,
}

impl Application {
    pub fn new() -> Self {
        let mut app = libhelium::Application::new(Some(APP_ID), Default::default());

        // placeholder window
        app.connect_activate(|app| {
            /*             let fleet = Fleet::new(app.clone());
            app.add_window(&fleet.fleet);
            fleet.fleet.present(); */
            let fleet = fleet::Fleet::new();
            app.add_window(&fleet);
            // fleet.set_application(Some(app));
            //     ^ ❓???? doesn't satisfy `fleet::Fleet: IsA<gtk4::Window>`
            fleet.activate_default();
        });

        Application { app }
    }

    pub fn run(&self) {
        self.app.run();
    }
}

// Fleet, the "panel" of the shell
// todo: rewrite this entire thing to make use of the blueprint template
/* pub struct Fleet {
    pub fleet: libhelium::ApplicationWindow,
}

impl Fleet {
    pub fn new(app: libhelium::Application) -> Self {
        let fleet = libhelium::ApplicationWindow::new(&app);
        fleet.set_title(Some("Fleet"));
        fleet.set_default_size(800, 40);
        fleet.init_layer_shell();
        fleet.set_layer(Layer::Top);
        fleet.auto_exclusive_zone_enable();

        let anchors = [
            (Edge::Top, true),
            (Edge::Bottom, false),
            (Edge::Left, true),
            (Edge::Right, true),
        ];

        for (edge, state) in anchors.iter() {
            fleet.set_anchor(*edge, *state);
        }

        Fleet { fleet }
    }
}
 */
// test template
/* use glib::Object;
use gtk::{gio, glib};


#[derive(CompositeTemplate, Default)]
#[template(file = "src/ui/fleet.blp")]
pub struct FleetTemplate {
    // #[template_child]
    // pub gtkbox: TemplateChild<gtk::Box>,
}

#[glib::object_subclass]
impl ObjectSubclass for FleetTemplate {
    const NAME: &'static str = "FleetTemplate";
    type Type = super::Fleet;
    type ParentType = gtk::Window;

    fn new() -> Self {
        Self::default()
    }
}
 */

mod fleet {
    use glib::subclass::object::ObjectImpl;
    use gtk::subclass::widget::{CompositeTemplateClass, WidgetImpl};
    use libhelium::{ffi::HeApplication, subclass::{application_window::HeApplicationWindowImpl, window::HeWindowImpl}};

    use super::*;
    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(file = "src/ui/fleet.blp")]
    pub struct Fleet {
        #[template_child]
        pub gtkbox: TemplateChild<gtk::Box>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Fleet {
        const NAME: &'static str = "Fleet";
        type Type = super::Fleet;
        type ParentType = libhelium::ApplicationWindow;

        fn new() -> Self {
            Self::default()
        }
        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Fleet {
        fn dispose(&self) {
            while let Some(child) = self.obj().first_child() {
                child.unparent();
            }
        }
    }
    impl WidgetImpl for Fleet {}
    impl HeApplicationWindowImpl for Fleet {}
    impl ApplicationWindowImpl for Fleet {}
    impl WindowImpl for Fleet {}
    impl HeWindowImpl for Fleet {}
    // unsafe impl IsA<gtk::Window> for Fleet {}
}

glib::wrapper! {
    // how the fuck am i supposed to make it consider this a gtk::Window
    pub struct Fleet(ObjectSubclass<fleet::Fleet>) @extends libhelium::ApplicationWindow, gtk::Window, gtk::Widget, libhelium::Window;
}
