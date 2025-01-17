use std::cell::LazyCell;
use std::rc::Rc;

use ballad_config::{ShellConfig, ThemeConfig, ThemeSelection};
use ballad_services::config::CONFIG_SERVICE;
use ballad_services::variable::Variable;
use gtk::glib::{self, clone};
use gtk::{Align, Box, Image, Label, Orientation, ScrolledWindow};
use gtk::{Button, prelude::*};

use super::config::{dark_theme_toggle_variable, on_theme_button_press};
use super::dropdown_button::DropdownButton;

fn set_button_classes(flavor: &ThemeSelection, button: &Button, retained_flavor: &ThemeSelection) {
    let is_active = *retained_flavor == *flavor;
    if is_active {
        button.add_css_class("active");
    } else {
        button.remove_css_class("active");
    }
}

fn theme_option_button(theme_selection: ThemeSelection, retained_selection: Variable) -> Button {
    let theme = Rc::new(theme_selection.theme().unwrap());

    let button = Button::builder()
        .child(
            &Label::builder()
                .label(&theme.name)
                .halign(Align::Start)
                .build(),
        )
        .css_classes(["toggle-button-dropdown-option"])
        .build();

    let theme_selection2 = theme_selection.clone();
    CONFIG_SERVICE.with(|service| {
        let service = LazyCell::force(service).clone();

        button.connect_clicked(clone!(
            #[strong]
            retained_selection,
            #[weak]
            service,
            #[strong]
            theme_selection2,
            move |_| {
                retained_selection.set_value_typed(theme_selection2.clone());

                service.set_shell_config(ShellConfig {
                    theme: ThemeConfig {
                        selected_theme: theme_selection2.clone(),
                        ..service.shell_config().theme
                    },
                    ..service.shell_config()
                });
            }
        ));

        retained_selection.connect_value_changed_typed(
            false,
            clone!(
                #[weak]
                button,
                #[weak]
                retained_selection,
                move |_: Variable, _: ThemeSelection| {
                    set_button_classes(
                        &theme_selection2,
                        &button,
                        &retained_selection.value_typed().unwrap(),
                    );
                }
            ),
        );
        set_button_classes(
            &theme_selection.clone(),
            &button,
            &retained_selection.value_typed().unwrap(),
        );
    });

    button
}

pub fn flavor_selector() -> Box {
    let toggled = dark_theme_toggle_variable();
    let retained_dark_flavor = Variable::with_value(CONFIG_SERVICE.with(|service| {
        let config = service.shell_config();
        if config.theme.selected_theme.is_dark().unwrap_or(true) {
            config.theme.selected_theme
        } else {
            ThemeSelection::default()
        }
        .into()
    }));
    let retained_light_flavor = Variable::with_value(CONFIG_SERVICE.with(|service| {
        let config = service.shell_config();
        if config.theme.selected_theme.is_light().unwrap_or(false) {
            config.theme.selected_theme
        } else {
            ThemeSelection::CatppuccinLatte
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

    let dark_theme_buttons = Rc::new(
        ballad_config::theme::get_or_init_all_theme_selections()
            .into_iter()
            .filter(|theme| theme.theme().is_some_and(|theme| theme.is_dark()))
            .map(|theme| theme_option_button(theme, retained_dark_flavor.clone()))
            .collect::<Vec<_>>(),
    );
    let light_theme_buttons = Rc::new(
        ballad_config::theme::get_or_init_all_theme_selections()
            .into_iter()
            .filter(|theme| theme.theme().is_some_and(|theme| theme.is_light()))
            .map(|theme| theme_option_button(theme, retained_light_flavor.clone()))
            .collect::<Vec<_>>(),
    );

    let flavor_options = Box::builder()
        .orientation(Orientation::Vertical)
        .halign(Align::Start)
        .build();
    let flavor_options_scroller = ScrolledWindow::builder()
        .min_content_height(64)
        .child(&flavor_options)
        .build();

    if toggled.value_typed().unwrap_or(true) {
        for button in dark_theme_buttons.iter() {
            flavor_options.append(button);
        }
    } else {
        for button in light_theme_buttons.iter() {
            flavor_options.append(button);
        }
    }

    toggled.connect_value_changed_typed(
        false,
        clone!(
            #[weak]
            flavor_options,
            #[strong]
            dark_theme_buttons,
            #[strong]
            light_theme_buttons,
            move |_, toggled: bool| {
                if toggled {
                    for button in light_theme_buttons.iter() {
                        flavor_options.remove(button);
                    }
                    for button in dark_theme_buttons.iter() {
                        flavor_options.append(button);
                    }
                } else {
                    for button in dark_theme_buttons.iter() {
                        flavor_options.remove(button);
                    }
                    for button in light_theme_buttons.iter() {
                        flavor_options.append(button);
                    }
                }
            }
        ),
    );

    DropdownButton::builder()
        .on_toggle(on_theme_button_press(
            retained_light_flavor,
            retained_dark_flavor,
        ))
        .toggled(toggled)
        .button_content(flavor_button_content)
        .dropdown_content(flavor_options_scroller)
        .build()
}
