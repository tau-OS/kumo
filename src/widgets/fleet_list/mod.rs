//! FleetWidgetList is a container for widgets for the Fleet.
//! 
//! It's derived from gtk::Box and is used to store widgets

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

    // pub fn add_widget(&self, widget: &gtk::Widget) {
    //     self.append(widget);
    // }
}
