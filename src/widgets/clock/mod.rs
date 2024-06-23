use std::time::Duration;

use chrono::prelude::*;
use glib::subclass::types::ObjectSubclassExt;
use gtk::prelude::WidgetExt;
mod imp;
#[derive(Default)]
pub enum TimeFormat {
    #[default]
    TwentyFourHour,
    TwelveHour,
}

glib::wrapper! {
    pub struct Clock(ObjectSubclass<imp::Clock>) @extends gtk::Box, gtk::Widget;
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

impl Default for Clock {
    fn default() -> Self {
        Self::new()
    }
}

impl Clock {
    pub fn new() -> Self {
        let obj: Self = glib::Object::new();

        const TIME_FORMAT: &str = "12h";
        // connect on realization

        obj.connect_realize(|clock| {
            let clock = clock.clone(); // Clone the reference to ensure it lives long enough
                                       // first tick is immediate to avoid a delay
            clock.tick(TIME_FORMAT);
            glib::timeout_add_local(Duration::from_millis(500), move || {
                clock.tick(TIME_FORMAT);
                glib::ControlFlow::Continue
            });
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
