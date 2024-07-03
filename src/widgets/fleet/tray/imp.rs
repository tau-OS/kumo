use appindicator3::{prelude::AppIndicatorExt, Indicator};
use glib::subclass::{
    object::{ObjectImpl, ObjectImplExt},
    types::ObjectSubclass,
};
use gtk::{
    prelude::{BoxExt, ButtonExt},
    subclass::{box_::BoxImpl, widget::WidgetImpl},
};

/// A representation of an AppIndicator icon
/// Should be clickable and have a context menu
pub struct SystemTrayItem {
    pub indicator: Indicator,
    pub icon: gtk::Image,
    pub menu: gtk::PopoverMenu,

    /// The box to contain all the widgets inside here
    box_: gtk::Box,
}

impl From<Indicator> for SystemTrayItem {
    fn from(indicator: Indicator) -> Self {
        let icon = gtk::Image::new();
        // todo: cleanup
        let icon_name = indicator.icon_name();
        let icon_name = icon_name.as_deref().unwrap_or("missing-icon");
        icon.set_from_icon_name(Some(icon_name));
        let menu = gtk::PopoverMenu::builder().build();
        let box_ = gtk::Box::new(gtk::Orientation::Vertical, 0);
        box_.append(&icon);
        Self {
            icon,
            indicator,
            menu,
            box_,
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for SystemTrayItem {
    const NAME: &'static str = "FleetSystemTrayEntry";
    type Type = super::SystemTrayItem;
    type ParentType = gtk::Box;

    fn new() -> Self {
        todo!()
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
