use std::cell::LazyCell;

use ballad_config::{CatppuccinFlavor, ShellConfig, ThemeConfig};
use ballad_services::config::CONFIG_SERVICE;
use ballad_services::variable::Variable;
use gtk::glib::{self, clone};
use gtk::{Align, Box, Image, Label, Orientation};
use gtk::{Button, prelude::*};

use super::config::{dark_theme_toggle_variable, on_theme_button_press};
use super::dropdown_button::DropdownButton;

pub fn flavor_selector() -> Box {
    let toggled = dark_theme_toggle_variable();
    let retained_dark_flavor = Variable::with_value(CONFIG_SERVICE.with(|service| {
        let config = service.shell_config();
        if config.theme.catppuccin_flavor.is_dark() {
            config.theme.catppuccin_flavor
        } else {
            CatppuccinFlavor::default()
        }
        .into()
    }));

    let flavor_button_content = Box::builder()
        .orientation(Orientation::Horizontal)
        .halign(Align::Start)
        .spacing(8)
        .build();
    flavor_button_content.append(
        &Image::builder()
            .icon_name("theme-symbolic")
            .pixel_size(24)
            .build(),
    );
    flavor_button_content.append(
        &Label::builder()
            .label("Dark Mode")
            .vexpand(true)
            .valign(Align::Center)
            .build(),
    );

    const DARK_CATPPUCCIN_FLAVORS: &[(CatppuccinFlavor, &str)] = &[
        (CatppuccinFlavor::Frappe, "Frappé"),
        (CatppuccinFlavor::Macchiato, "Macchiato"),
        (CatppuccinFlavor::Mocha, "Mocha"),
    ];
    let flavor_options = Box::builder()
        .orientation(Orientation::Vertical)
        .halign(Align::Start)
        .build();
    for flavor in DARK_CATPPUCCIN_FLAVORS {
        let button = Button::builder()
            .child(
                &Label::builder()
                    .label(flavor.1)
                    .halign(Align::Start)
                    .build(),
            )
            .css_classes(["toggle-button-dropdown-option"])
            .build();

        CONFIG_SERVICE.with(|service| {
            let service = LazyCell::force(service).clone();

            button.connect_clicked(clone!(
                #[weak]
                retained_dark_flavor,
                #[weak]
                service,
                move |_| {
                    retained_dark_flavor.set_value_typed(flavor.0);
                    if service.shell_config().theme.catppuccin_flavor.is_dark() {
                        service.set_shell_config(ShellConfig {
                            theme: ThemeConfig {
                                catppuccin_flavor: flavor.0,
                            },
                            ..service.shell_config()
                        });
                    }
                }
            ));

            fn set_button_classes(
                flavor: &CatppuccinFlavor,
                button: &Button,
                retained_flavor: &CatppuccinFlavor,
            ) {
                let is_active = *retained_flavor == *flavor;
                if is_active {
                    button.add_css_class("active");
                } else {
                    button.remove_css_class("active");
                }
            }

            retained_dark_flavor.connect_value_changed_typed(
                false,
                clone!(
                    #[weak]
                    button,
                    #[weak]
                    retained_dark_flavor,
                    move |_: Variable, _: CatppuccinFlavor| {
                        set_button_classes(
                            &flavor.0,
                            &button,
                            &retained_dark_flavor.value_typed().unwrap(),
                        );
                    }
                ),
            );
            set_button_classes(
                &flavor.0,
                &button,
                &retained_dark_flavor.value_typed().unwrap(),
            );
        });

        flavor_options.append(&button);
    }

    DropdownButton::builder()
        .on_toggle(on_theme_button_press(retained_dark_flavor))
        .toggled(toggled)
        .button_content(flavor_button_content)
        .dropdown_content(flavor_options)
        .build()
}
