use glib::subclass::object::ObjectImpl;
use gtk::subclass::widget::{CompositeTemplateClass, WidgetImpl};
use libhelium::subclass::{application_window::HeApplicationWindowImpl, window::HeWindowImpl};

use super::*;
#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(file = "src/widgets/bar/bar.blp")]
pub struct Bar {
    #[template_child(id = "iconlist")]
    pub gtkbox: TemplateChild<gtk::Box>,
}

#[glib::object_subclass]
impl ObjectSubclass for Bar {
    const NAME: &'static str = "Bar";
    type Type = super::Bar;
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

impl ObjectImpl for Bar {
    fn dispose(&self) {
        while let Some(child) = self.obj().first_child() {
            child.unparent();
        }
    }
}
impl WidgetImpl for Bar {}
impl HeApplicationWindowImpl for Bar {}
impl ApplicationWindowImpl for Bar {}
impl WindowImpl for Bar {}
impl HeWindowImpl for Bar {}
// unsafe impl IsA<gtk::Window> for Fleet {}

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(file = "src/widgets/bar/appicon.blp")]
pub struct AppIcon {}
#[glib::object_subclass]
impl ObjectSubclass for AppIcon {
    const NAME: &'static str = "AppIcon";
    type Type = super::AppIcon;
    type ParentType = gtk::Button;

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
impl ObjectImpl for AppIcon {}
impl ButtonImpl for AppIcon {}
impl WidgetImpl for AppIcon {}
