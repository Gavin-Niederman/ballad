use gtk::{
    ApplicationWindow, Box,
    cairo::{RectangleInt, Region},
    prelude::*,
};
use gtk4_layer_shell::{KeyboardMode, LayerShell};

use crate::widgets::{window::{window, Anchor, WindowProperties}, PerMonitorWidgetProperties};

pub fn screen_bevels(
    PerMonitorWidgetProperties {
        monitor,
        application,
    }: PerMonitorWidgetProperties,
) -> ApplicationWindow {
    let window = window(
        WindowProperties::builder()
            .anchors(&[Anchor::Left, Anchor::Right, Anchor::Top, Anchor::Bottom])
            .application(application)
            .title(&format!("screen-bevels-{}", monitor.connector().unwrap()))
            .monitor(monitor)
            .build(),
    );
    window.set_hexpand(true);
    window.set_vexpand(true);
    window.set_css_classes(&["screen-bevels"]);
    window.set_keyboard_mode(KeyboardMode::None);

    window.connect_realize(|window| {
        let surface = window.surface().unwrap();
        // enable click through
        surface.set_input_region(&Region::create_rectangle(&RectangleInt::new(0, 0, 0, 0)));
    });

    let shadow = Box::builder()
        .css_classes(["shadow"])
        .vexpand(true)
        .hexpand(true)
        .build();
    let bevels = Box::builder()
        .css_classes(["bevels"])
        .vexpand(true)
        .hexpand(true)
        .build();

    shadow.append(&bevels);
    window.set_child(Some(&shadow));

    window
}
