pub mod battery;
pub mod screen_bevels;

use ballad_services::battery::BATTERY_SERVICE;
use gtk::{
    Align, ApplicationWindow, Box, CenterBox, Orientation,
    prelude::{BoxExt, GtkWindowExt, MonitorExt},
};

use super::{window::{window, Anchor, WindowProperties}, PerMonitorWidgetProperties};

pub fn sidebar(
    PerMonitorWidgetProperties {
        monitor,
        application,
    }: PerMonitorWidgetProperties,
) -> ApplicationWindow {
    let window = window(
        WindowProperties::builder()
            .anchors(&[Anchor::Left, Anchor::Top, Anchor::Bottom])
            .application(application)
            .title(&format!("sidebar-{}", monitor.connector().unwrap()))
            .monitor(monitor)
            .auto_exclusive(true)
            .build(),
    );

    let container = CenterBox::builder()
        .css_classes(["sidebar-container"])
        .name("sidebar-container")
        .orientation(Orientation::Vertical)
        .build();

    let lower_section = Box::builder()
        .name("lower-widgets-section")
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
