use std::time::Duration;

use chrono::prelude::*;
use glib::{subclass::types::ObjectSubclassExt, ObjectExt};
mod imp;
pub enum TimeFormat {
    TwentyFourHour,
    TwelveHour,
}

glib::wrapper! {
    pub struct Clock(ObjectSubclass<imp::Clock>) @extends gtk::Box, gtk::Widget;
}

impl Default for TimeFormat {
    fn default() -> Self {
        TimeFormat::TwentyFourHour
    }
}

impl TimeFormat {
    pub fn format_time(&self, time: DateTime<Local>) -> String {
        match self {
            TimeFormat::TwentyFourHour => time.format("%H:%M:%S").to_string(),
            TimeFormat::TwelveHour => time.format("%I:%M:%S %p").to_string(),
        }
    }
}

// From value string
impl From<&str> for TimeFormat {
    fn from(s: &str) -> Self {
        match s {
            "24h" => TimeFormat::TwentyFourHour,
            "12h" => TimeFormat::TwelveHour,
            _ => TimeFormat::TwentyFourHour,
        }
    }
}

impl Clock {
    pub fn new() -> Self {
        let obj: Self = glib::Object::new();

        // connect on realization

        obj.connect_local("realize", false, move |args| {
            let this = args[0].get::<Clock>().unwrap();
            let format = args[1].get::<String>().unwrap();
            glib::timeout_add_local(Duration::from_millis(500), move || {
                this.tick(&format);
                glib::ControlFlow::Continue
            });
            None
        });
        obj
    }

    pub fn downcast(&self) -> &imp::Clock {
        imp::Clock::from_obj(self)
    }

    pub fn tick(&self, format: &str) {
        imp::Clock::on_clock_tick(self.downcast(), format);
    }
}
