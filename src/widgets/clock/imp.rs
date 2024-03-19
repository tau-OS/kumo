use std::cell::Cell;

use glib::subclass::object::ObjectImpl;
use glib::subclass::types::ObjectSubclass;
use gtk::subclass::box_::BoxImpl;
use gtk::subclass::widget::{CompositeTemplateClass, WidgetImpl};
use gtk::subclass::widget::{CompositeTemplateInitializingExt, WidgetClassExt};
use gtk::TemplateChild;

use super::TimeFormat;

#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(file = "src/widgets/clock/clock.blp")]
pub struct Clock {
    #[template_child(id = "time")]
    pub clock_label: TemplateChild<gtk::Label>,
    pub time: Cell<chrono::DateTime<chrono::Local>>,
}

#[glib::object_subclass]
impl ObjectSubclass for Clock {
    const NAME: &'static str = "KumoClock";
    type Type = super::Clock;
    type ParentType = gtk::Box;

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

#[gtk::template_callbacks]
impl Clock {
    #[template_callback]
    pub fn on_clock_tick(&self, format: &str) {
        let ttime = chrono::Local::now();
        let tformat = TimeFormat::from(format);
        self.time.set(ttime);
        self.clock_label
            .set_text(&tformat.format_time(self.time.get()).to_string());
    }
}

impl ObjectImpl for Clock {}

impl WidgetImpl for Clock {}

impl BoxImpl for Clock {}
