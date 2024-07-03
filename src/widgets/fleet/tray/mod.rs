//! System tray Fleet component.
//! 
//! This module contains the AppIndicator/StatusNotifierItem implementation for the Fleet.
//! 


use glib::Cast;
use gtk::prelude::BoxExt;

mod imp;

glib::wrapper! {
    pub struct SystemTray(ObjectSubclass<imp::SystemTray>) @extends gtk::Box;
}

glib::wrapper! {
    pub struct SystemTrayItem(ObjectSubclass<imp::SystemTrayItem>) @extends gtk::Box;
}

impl Default for SystemTray {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemTray {
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
