use std::time::Duration;

use ballad_config::CatppuccinFlavor;
use ballad_config::ThemeConfig;
use ballad_services::config::CONFIG_SERVICE;
use ballad_services::config::ConfigService;
use gtk::Align;
use gtk::Label;
use gtk::Orientation;
use gtk::glib::ControlFlow;
use gtk::glib::clone;
use gtk::glib::{self, closure_local};
use gtk::prelude::*;
use gtk::{ApplicationWindow, Box};

use super::window::Anchor;
use super::{
    PerMonitorWidget,
    window::{Layer, LayershellWindow},
};

fn formatted_time() -> String {
    let time = chrono::Local::now();
    time.format("%T").to_string()
}
fn formatted_date() -> String {
    let date = chrono::Local::now();
    date.format("%A, %B %-d, %-y").to_string()
}

pub fn clock_underlay(
    PerMonitorWidget {
        monitor,
        application,
    }: PerMonitorWidget,
) -> ApplicationWindow {
    let window: ApplicationWindow = LayershellWindow::builder()
        .application(application)
        .title(&format!("clock-underlay-{}", monitor.connector().unwrap()))
        .monitor(monitor)
        .layer(Layer::Bottom)
        .anchors(&[Anchor::Bottom, Anchor::Right])
        .build();

    let clock = Box::builder()
        .css_classes(["clock"])
        .name("clock")
        .orientation(Orientation::Vertical)
        .build();

    let time = Label::builder()
        .css_classes(["time"])
        .name("time")
        .label(formatted_time())
        .build();
    let date = Label::builder()
        .css_classes(["date"])
        .name("date")
        .label(formatted_date())
        .halign(Align::Start)
        .build();

    glib::timeout_add_local(
        Duration::from_secs(1),
        clone!(
            #[weak]
            time,
            #[weak]
            date,
            #[upgrade_or]
            ControlFlow::Break,
            move || {
                time.set_label(&formatted_time());
                date.set_label(&formatted_date());
                ControlFlow::Continue
            }
        ),
    );

    CONFIG_SERVICE.with(|service| {
        fn clock_underlay_classes(config: &ThemeConfig) -> Vec<&str> {
            vec!["clock-underlay", match config.catppuccin_flavor {
                CatppuccinFlavor::Latte => "light",
                _ => "dark",
            }]
        }

        service.connect_closure("shell-theme-config-changed", false, {
            closure_local!(
                #[weak]
                window,
                move |_: ConfigService, config: &ThemeConfig| {
                    let classes = clock_underlay_classes(config);
                    window.set_css_classes(&classes);
                }
            )
        });
        window.set_css_classes(&clock_underlay_classes(&service.shell_config().theme));
    });

    clock.append(&time);
    clock.append(&date);
    window.set_child(Some(&clock));

    window
}
