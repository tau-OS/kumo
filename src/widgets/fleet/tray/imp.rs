use glib::subclass::{object::{ObjectImpl, ObjectImplExt}, types::ObjectSubclass};
use gtk::{prelude::{BoxExt, ButtonExt}, subclass::{box_::BoxImpl, widget::WidgetImpl}};

/// A representation of an AppIndicator icon
/// Should be clickable and have a context menu
pub struct SystemTrayItem {
    pub icon: gtk::Image,
    pub menu: gtk::PopoverMenu,

    /// The box to contain all the widgets inside here
    box_: gtk::Box,
}

#[glib::object_subclass]
impl ObjectSubclass for SystemTrayItem {
    const NAME: &'static str = "FleetSystemTrayEntry";
    type Type = super::SystemTrayItem;
    type ParentType = gtk::Box;

    fn new() -> Self {
        Self {
            icon: gtk::Image::new(),
            menu: gtk::PopoverMenu::builder().build(),
            box_: gtk::Box::new(gtk::Orientation::Vertical, 0),
        }
    }
}

impl BoxImpl for SystemTrayItem {}
impl WidgetImpl for SystemTrayItem {}
impl ObjectImpl for SystemTrayItem {
    fn constructed(&self) {
        self.parent_constructed();
        let button = libhelium::FillButton::new("mrrow");
        button.set_child(Some(&self.icon));
        self.box_.append(&button)
    }
}


pub struct SystemTray {
    // Box of SystemTrayIcons
    pub box_: gtk::Box,
}

#[glib::object_subclass]
impl ObjectSubclass for SystemTray {
    const NAME: &'static str = "FleetSystemTray";
    type Type = super::SystemTray;
    type ParentType = gtk::Box;

    fn new() -> Self {
        Self {
            box_: gtk::Box::new(gtk::Orientation::Horizontal, 0),
        }
    }
}

impl BoxImpl for SystemTray {}
impl WidgetImpl for SystemTray {}
impl ObjectImpl for SystemTray {}
