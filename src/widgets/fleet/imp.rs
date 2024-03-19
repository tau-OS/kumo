use crate::widgets;
use super::*;
use glib::subclass::object::ObjectImpl;
// use glib::ControlFlow::Continue;
use gtk::subclass::widget::{CompositeTemplateClass, WidgetImpl};
use libhelium::subclass::{application_window::HeApplicationWindowImpl, window::HeWindowImpl};
#[derive(Debug, Default, gtk::CompositeTemplate)]
#[template(file = "src/widgets/fleet/fleet.blp")]
pub struct Fleet {
    #[template_child(id = "appbox")]
    pub gtkbox: TemplateChild<gtk::Box>,

    // #[template_child(id = "clock")]
    // pub clock: TemplateChild<gtk::Label>,
    // pub time: Cell<chrono::DateTime<chrono::Local>>,
    #[template_child(id = "clockbox")]
    pub clock: TemplateChild<gtk::Box>,
}

#[glib::object_subclass]
impl ObjectSubclass for Fleet {
    const NAME: &'static str = "Fleet";
    type Type = super::Fleet;
    type ParentType = libhelium::ApplicationWindow;

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

// #[gtk::template_callbacks]
// impl Fleet {
//     #[template_callback]
//     pub fn on_clock_tick(&self) {
//         self.time.set(chrono::Local::now());
//         let timestring = &self.time.get().format("%H:%M:%S").to_string();
//         self.clock.set_width_chars(timestring.len() as i32 + 2);
//         self.clock
//             .set_text(timestring);
//     }
// }

impl ObjectImpl for Fleet {
    fn dispose(&self) {
        while let Some(child) = self.obj().first_child() {
            child.unparent();
        }
    }

    fn constructed(&self) {
        self.parent_constructed();

        let clock = widgets::clock::Clock::new();

        // don't set child, actually literally copy clock into the box
        self.clock.append(&clock);
    }
}
impl WidgetImpl for Fleet {}
impl HeApplicationWindowImpl for Fleet {}
impl ApplicationWindowImpl for Fleet {}
impl WindowImpl for Fleet {}
impl HeWindowImpl for Fleet {}
// unsafe impl IsA<gtk::Window> for Fleet {}
