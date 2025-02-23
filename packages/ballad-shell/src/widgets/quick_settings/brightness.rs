use std::cell::LazyCell;

use ballad_services::brightness::BRIGHTNESS_SERVICE;
use gtk::glib;
use gtk::{Scale, glib::clone, prelude::*};

use crate::widgets::icon::symbolic_icon;

pub fn brightness() -> gtk::Box {
    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .name("brightness-container")
        .css_classes(["brightness"])
        .hexpand(true)
        .spacing(4)
        .build();

    let brightness_bar = Scale::builder()
        .orientation(gtk::Orientation::Horizontal)
        .css_classes(["brightness-bar", "horizontal"])
        .name("brightness-bar")
        .hexpand(true)
        .build();
    brightness_bar.set_range(0.0, 1.0);

    container.append(&symbolic_icon("brightness-symbolic", 24));
    container.append(&brightness_bar);

    BRIGHTNESS_SERVICE.with(|service| {
        let service = LazyCell::force(service).clone();

        brightness_bar.set_value(service.brightness_blocking());

        service.connect_brightness(clone!(
            #[weak]
            brightness_bar,
            move |_, real_brightness| {
                brightness_bar.set_value(real_brightness);
            }
        ));
        brightness_bar.connect_value_changed(move |bar| {
            service.set_brightness(bar.value());
        })
    });

    container
}
