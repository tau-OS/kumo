use glib::subclass::{object::ObjectImpl, types::ObjectSubclass};
use gtk::subclass::{box_::BoxImpl, widget::WidgetImpl};

pub struct FleetWidgetList {
    pub box_: gtk::Box,
}

#[glib::object_subclass]
impl ObjectSubclass for FleetWidgetList {
    const NAME: &'static str = "FleetWidgetList";
    type Type = super::FleetWidgetList;
    type ParentType = gtk::Box;

    fn new() -> Self {
        Self {
            box_: gtk::Box::new(gtk::Orientation::Horizontal, 0),
        }
    }
}

impl BoxImpl for FleetWidgetList {}
impl WidgetImpl for FleetWidgetList {}
impl ObjectImpl for FleetWidgetList {}


