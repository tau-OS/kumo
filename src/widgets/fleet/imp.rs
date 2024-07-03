use super::*;
use fleet_list::FleetWidgetList;
use glib::subclass::object::ObjectImpl;
// use glib::ControlFlow::Continue;
use gtk::subclass::widget::{CompositeTemplateClass, WidgetImpl};
use libhelium::subclass::{application_window::HeApplicationWindowImpl, window::HeWindowImpl};
#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(file = "src/widgets/fleet/fleet.blp")]
pub struct Fleet {
    #[template_child(id = "appbox")]
    pub gtkbox: TemplateChild<gtk::Box>,

    // #[template_child(id = "clock")]
    // pub clock: TemplateChild<gtk::Label>,
    // pub time: Cell<chrono::DateTime<chrono::Local>>,
    pub widgets: FleetWidgetList,
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
        // klass.bind_template_callbacks();
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

    // todo: make configurable!
    fn constructed(&self) {
        self.parent_constructed();

        self.widgets.append(&clock::Clock::new());

        // we should clean this up and put it inside the blueprint
        self.gtkbox.append(self.widgets.as_box());
    }
}
impl WidgetImpl for Fleet {}
impl HeApplicationWindowImpl for Fleet {}
impl ApplicationWindowImpl for Fleet {}
impl WindowImpl for Fleet {}
impl HeWindowImpl for Fleet {}
// unsafe impl IsA<gtk::Window> for Fleet {}
