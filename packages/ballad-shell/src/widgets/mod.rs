use gtk::{gdk::Monitor, Application};
use typed_builder::TypedBuilder;

pub mod sidebar;
pub mod window;
pub mod symbolic_icon;
pub mod clock_underlay;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Orientation {
    Vertical,
    Horizontal,
}
impl From<Orientation> for gtk::Orientation {
    fn from(orientation: Orientation) -> Self {
        match orientation {
            Orientation::Vertical => gtk::Orientation::Vertical,
            Orientation::Horizontal => gtk::Orientation::Horizontal,
        }
    }
}

#[derive(Debug, TypedBuilder, Clone, PartialEq, Eq)]
pub struct PerMonitorWidgetProperties<'a> {
    pub monitor: Monitor,
    pub application: &'a Application,
}