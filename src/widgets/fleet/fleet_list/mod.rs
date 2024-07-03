//! FleetWidgetList is a container for widgets for the Fleet.
//!
//! It's derived from gtk::Box and is used to store widgets

use glib::Cast;
use gtk::prelude::BoxExt;

mod imp;

glib::wrapper! {
    pub struct FleetWidgetList(ObjectSubclass<imp::FleetWidgetList>) @extends gtk::Box;
}

impl Default for FleetWidgetList {
    fn default() -> Self {
        Self::new()
    }
}

impl FleetWidgetList {
    pub fn new() -> Self {
        let obj: Self = glib::Object::new();
        obj
    }

    pub fn as_box(&self) -> &gtk::Box {
        &self.upcast_ref()
    }

    pub fn add_widget(&self, widget: &impl glib::IsA<gtk::Widget>) {
        self.as_box().append(widget)
    }
}
