use std::fmt::Debug;

pub mod clock;
pub mod fleet;
pub mod bar;
pub mod fleet_list;

#[deprecated = "Please make it a normal widget instead"]
pub trait FleetWidgetTrait {
    fn as_widget(&self) -> &gtk::Widget;
    
    fn update(&mut self) {}

    /// The priority of the widget. Lower numbers are higher priority.
    /// A widget list would be sorted by priority before being added to the bar.
    fn priority(&self) -> i32 {
        0
    }
}

#[deprecated = "Use the FleetWidgetList widget"]
#[derive(Default)]
pub struct FleetWidgets {
    widgets: Vec<Box<dyn FleetWidgetTrait>>,
}

impl Debug for FleetWidgets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FleetWidgetList")
            .field("widgets", &self.widgets.len())
            .finish()
    }
}

impl FleetWidgets {
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
        }
    }

    pub fn add_widget(&mut self, widget: Box<dyn FleetWidgetTrait>) {
        self.widgets.push(widget);
        self.sort();
    }

    pub fn sort(&mut self) {
        self.widgets.sort_by(|a, b| a.priority().cmp(&b.priority()));
    }
    
    pub fn update(&mut self) {
        for widget in self.widgets.iter_mut() {
            widget.update();
        }
    }
    
    pub fn as_widgets(&self) -> Vec<&gtk::Widget> {
        self.widgets.iter().map(|widget| widget.as_widget()).collect()
    }
}


