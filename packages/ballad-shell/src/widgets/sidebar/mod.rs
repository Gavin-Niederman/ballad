pub mod battery;
pub mod niri;
pub mod screen_bevels;
pub mod volume;

use std::cell::Cell;

use ballad_services::upower::UPOWER_SERVICE;
use gtk::glib;
use gtk::{
    Align, ApplicationWindow, Box, Button, CenterBox, Orientation, Separator, glib::clone,
    prelude::*,
};

use crate::app::APP;

use super::{
    PerMonitorWidget,
    icon::symbolic_icon,
    quick_settings::QUICK_SETTINGS_WINDOW_TITLE,
    window::{Anchor, LayershellWindow},
};

pub fn quick_settings_toggle() -> Button {
    let button = Button::builder()
        .name("quick-settings-toggle")
        .css_classes(["icon-container", "hoverable"])
        .build();

    button.connect_clicked(move |_| {
        if let Some(quick_settings_window) = APP.with(|app| {
            app.borrow()
                .as_ref()
                .map(|app| app.window_by_title(QUICK_SETTINGS_WINDOW_TITLE).unwrap())
        }) {
            quick_settings_window.set_visible(!quick_settings_window.is_visible())
        }
    });

    let icon = symbolic_icon("settings-symbolic", 24);
    button.set_child(Some(&icon));

    button
}

pub fn sidebar(
    PerMonitorWidget {
        monitor,
        application,
    }: PerMonitorWidget,
) -> ApplicationWindow {
    let window: ApplicationWindow = LayershellWindow::builder()
        .anchors(&[Anchor::Left, Anchor::Top, Anchor::Bottom])
        .application(application)
        .title(&format!("sidebar-{}", monitor.connector().unwrap()))
        .monitor(monitor.clone())
        .auto_exclusive(true)
        .build();

    let container = CenterBox::builder()
        .css_classes(["sidebar-container"])
        .name("sidebar-container")
        .orientation(Orientation::Vertical)
        .build();

    let upper_section = Box::builder()
        .name("upper-widgets-section")
        .orientation(Orientation::Vertical)
        .valign(Align::Start)
        .build();

    let windows = niri::windows();
    let workspaces = niri::workspaces(monitor);

    upper_section.append(&workspaces);
    upper_section.append(
        &Separator::builder()
            .orientation(Orientation::Vertical)
            .name("upper-widgets-seperator")
            .build(),
    );
    upper_section.append(&windows);

    let lower_section = Box::builder()
        .name("lower-widgets-section")
        .orientation(Orientation::Vertical)
        .valign(Align::End)
        .build();

    let quick_settings_toggle = quick_settings_toggle();
    let battery = battery::Battery::builder().build();
    let volume = volume::Volume::builder().build();

    lower_section.append(&quick_settings_toggle);
    lower_section.append(
        &Separator::builder()
            .orientation(Orientation::Vertical)
            .name("lower-widgets-seperator")
            .build(),
    );
    lower_section.append(&volume);
    UPOWER_SERVICE.with(clone!(
        #[strong]
        battery,
        #[weak]
        lower_section,
        move |service| {
            let added = Cell::new(false);
            service.connect_available_notify(move |service| {
                if service.available() && !added.get() {
                    lower_section.append(&battery);
                    added.set(true);
                }
            });
        }
    ));

    container.set_start_widget(Some(&upper_section));
    container.set_end_widget(Some(&lower_section));

    window.set_child(Some(&container));
    window
}
