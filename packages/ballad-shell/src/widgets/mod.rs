use gtk::{Application, gdk::Monitor};
use typed_builder::TypedBuilder;

pub mod clock;
pub mod icon;
pub mod quick_settings;
pub mod sidebar;
pub mod window;

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
pub struct PerMonitorWidget<'a> {
    pub monitor: Monitor,
    pub application: &'a Application,
}
