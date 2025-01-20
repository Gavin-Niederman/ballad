use std::cell::LazyCell;

use ballad_services::{power_profiles::POWER_PROFILES_SERVICE, reactive::Reactive};
use gtk::{Align, glib};
use gtk::{Button, Orientation, glib::clone};
use gtk::{Image, Label, ScrolledWindow, prelude::*};

use crate::utils::set_class_on_widget;

use super::dropdown_button::DropdownButton;

fn profile_option(name: String, retained_profile: Reactive<String>) -> Button {
    let button = Button::builder()
        .css_classes(["toggle-button-dropdown-option"])
        .hexpand(true)
        .halign(Align::Start)
        .label(&name)
        .build();
    set_class_on_widget(retained_profile.get_blocking() == name, &button, "active");

    let name2 = name.clone();
    retained_profile.connect(clone!(
        #[weak]
        button,
        move |_, active| {
            set_class_on_widget(active == name2, &button, "active");
        }
    ));
    button.connect_clicked(clone!(
        #[strong]
        retained_profile,
        move |_| {
            retained_profile.set_blocking(name.clone());
        }
    ));
    button
}

pub fn power_profile_selector() -> gtk::Box {
    let service = POWER_PROFILES_SERVICE.with(|service| LazyCell::force(service).clone());

    let pp_enabled = Reactive::new(true);
    let retained_profile = Reactive::new(service.active_profile_blocking());

    retained_profile.connect(clone!(
        #[weak]
        service,
        #[weak]
        pp_enabled,
        move |_, profile| {
            if pp_enabled.get_blocking() {
                service.set_active_profile(profile);
            }
        }
    ));
    pp_enabled.connect(clone!(
        #[weak]
        service,
        move |_, enabled| {
            if !enabled {
                service.set_active_profile("balanced".to_string());
            }
        }
    ));

    let pp_button_content = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .halign(Align::Start)
        .spacing(8)
        .build();
    pp_button_content.append(
        &Image::builder()
            .icon_name("gauge-symbolic")
            .pixel_size(24)
            .build(),
    );
    pp_button_content.append(
        &Label::builder()
            .label("Power Profiles")
            .vexpand(true)
            .valign(Align::Center)
            .build(),
    );

    let pp_options_content = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(4)
        .halign(Align::Start)
        .name("power-profile-options")
        .css_classes(["power-profile-options"])
        .build();

    for option in service.profiles_blocking() {
        pp_options_content.append(&profile_option(option, retained_profile.clone()));
    }

    let pp_options_scroller = ScrolledWindow::builder()
        .min_content_height(64)
        .child(&pp_options_content)
        .build();

    DropdownButton::builder()
        .on_toggle(|_| {})
        .toggled(pp_enabled)
        .button_content(pp_button_content)
        .dropdown_content(pp_options_scroller)
        .build()
}
