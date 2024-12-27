use gtk::{gdk::Monitor, prelude::{GtkWindowExt, MonitorExt, OrientableExt, WidgetExt}, Application, ApplicationWindow, CenterBox};
use typed_builder::TypedBuilder;

use super::window::{Anchor, WindowProperties, window};

#[derive(Debug, TypedBuilder, Clone, PartialEq, Eq)]
#[builder(field_defaults(default))]
pub struct SideBarProperties<'a> {
    #[builder(!default)]
    pub monitor: Monitor,
    #[builder(!default)]
    pub application: &'a Application,
}

pub fn sidebar(
    SideBarProperties {
        monitor,
        application,
    }: SideBarProperties,
) -> ApplicationWindow {
    let window = window(
        WindowProperties::builder()
            .anchors(&[Anchor::Left, Anchor::Top, Anchor::Bottom])
            .application(application)
            .title(&format!("sidebar-{}", monitor.connector().unwrap()))
            .monitor(monitor)
            .build(),
    );

    let container = CenterBox::new();
    container.set_orientation(gtk::Orientation::Vertical);
    container.set_css_classes(&["sidebar-container"]);
    
    window.set_child(Some(&container));
    window
}
