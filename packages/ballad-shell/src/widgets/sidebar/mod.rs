pub mod battery;

use ballad_services::battery::BATTERY_SERVICE;
use gtk::{
    Align, Application, ApplicationWindow, Box, CenterBox, Orientation,
    gdk::Monitor,
    prelude::{BoxExt, GtkWindowExt, MonitorExt, OrientableExt, WidgetExt},
};
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
    container.set_orientation(Orientation::Vertical);
    container.set_css_classes(&["sidebar-container"]);

    let lower_section = Box::builder()
        .orientation(Orientation::Vertical)
        .valign(Align::End)
        .build();

    let battery_available = BATTERY_SERVICE.with(|service| service.available());
    if battery_available {
        let battery = battery::battery(battery::BatteryProperties::builder().build());
        lower_section.append(&battery);
    }

    container.set_end_widget(Some(&lower_section));

    window.set_child(Some(&container));
    window
}
