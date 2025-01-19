use std::cell::{LazyCell, RefCell};
use std::rc::Rc;

use ballad_config::theme::ThemeVariant;
use ballad_config::{ShellConfig, ThemeConfig, ThemeSelection};
use ballad_services::config::CONFIG_SERVICE;
use ballad_services::reactive::Reactive;
use gtk::glib::{self, clone};
use gtk::{Align, Box, Image, Label, Orientation, ScrolledWindow};
use gtk::{Button, prelude::*};

use super::dropdown_button::DropdownButton;

fn set_button_classes(flavor: &ThemeSelection, button: &Button, retained_flavor: &ThemeSelection) {
    let is_active = *retained_flavor == *flavor;
    if is_active {
        button.add_css_class("active");
    } else {
        button.remove_css_class("active");
    }
}

pub fn dark_theme_toggle_variable() -> Reactive<bool> {
    let service = CONFIG_SERVICE.with(|service| LazyCell::force(service).clone());

    let initial = service
        .shell_config()
        .theme
        .selected_theme
        .is_dark()
        .unwrap_or_default();

    let variable = Reactive::new(initial);

    service.connect_shell_config_notify(clone!(
        #[strong]
        variable,
        move |config| {
            variable.set_blocking(
                config
                    .shell_config()
                    .theme
                    .selected_theme
                    .is_dark()
                    .unwrap_or_default(),
            );
        }
    ));

    variable
}

pub fn on_theme_button_press(
    retained_light_flavor: Reactive<ThemeSelection>,
    retained_dark_flavor: Reactive<ThemeSelection>,
) -> impl Fn(bool) + 'static {
    CONFIG_SERVICE.with(|config_service| {
        let config_service = LazyCell::force(config_service).clone();

        move |dark: bool| {
            let current_config = config_service.shell_config();

            if !dark {
                config_service.set_shell_config(ShellConfig {
                    theme: ThemeConfig {
                        selected_theme: retained_light_flavor.get_blocking(),
                        ..current_config.theme
                    },
                    ..current_config
                });
            } else {
                config_service.set_shell_config(ShellConfig {
                    theme: ThemeConfig {
                        selected_theme: retained_dark_flavor.get_blocking(),
                        ..current_config.theme
                    },
                    ..current_config
                });
            }
        }
    })
}

fn theme_option_button(
    theme_selection: ThemeSelection,
    retained_selection: Reactive<ThemeSelection>,
) -> Button {
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
                retained_selection.set_blocking(theme_selection2.clone());

                service.set_shell_config(ShellConfig {
                    theme: ThemeConfig {
                        selected_theme: theme_selection2.clone(),
                        ..service.shell_config().theme
                    },
                    ..service.shell_config()
                });
            }
        ));

        retained_selection.connect(clone!(
            #[weak]
            button,
            #[upgrade_or_default]
            move |_, selection| {
                set_button_classes(&theme_selection2, &button, &selection);
                Default::default()
            }
        ));
        set_button_classes(
            &theme_selection.clone(),
            &button,
            &retained_selection.get_blocking(),
        );
    });

    button
}

fn variant_options(variant: ThemeVariant, retained_selection: Reactive<ThemeSelection>) -> Box {
    let service = CONFIG_SERVICE.with(|service| LazyCell::force(service).clone());
    let container = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(8)
        .halign(Align::Start)
        .build();
    let buttons = Rc::new(RefCell::new(Vec::new()));

    fn update_options(
        container: &Box,
        variant: ThemeVariant,
        retained_selection: Reactive<ThemeSelection>,
        buttons: Rc<RefCell<Vec<Button>>>,
    ) {
        for button in buttons.borrow_mut().iter() {
            container.remove(button);
        }
        buttons.borrow_mut().clear();

        let theme_options = ballad_config::theme::get_or_init_all_theme_selections()
            .unwrap_or_default()
            .into_iter()
            .filter(|theme| theme.theme().is_some_and(|theme| theme.variant == variant))
            .map(|theme| theme_option_button(theme, retained_selection.clone()))
            .collect::<Vec<_>>();

        for button in theme_options.iter() {
            buttons.borrow_mut().push(button.clone());
            container.append(button);
        }
    }

    update_options(
        &container,
        variant,
        retained_selection.clone(),
        buttons.clone(),
    );

    service.connect_shell_config_notify(clone!(
        #[strong]
        container,
        #[weak]
        retained_selection,
        #[strong]
        buttons,
        move |_| {
            update_options(&container, variant, retained_selection, buttons.clone());
        }
    ));

    container
}

pub fn flavor_selector() -> Box {
    let service = CONFIG_SERVICE.with(|service| LazyCell::force(service).clone());

    let toggled = dark_theme_toggle_variable();
    let retained_dark_flavor = Reactive::new({
        let config = service.shell_config();
        if config.theme.selected_theme.is_dark().unwrap_or(true) {
            config.theme.selected_theme
        } else {
            ThemeSelection::default()
        }
    });
    let retained_light_flavor = Reactive::new({
        let config = service.shell_config();
        if config.theme.selected_theme.is_light().unwrap_or(false) {
            config.theme.selected_theme
        } else {
            ThemeSelection::CatppuccinLatte
        }
    });

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

    let light_options = variant_options(ThemeVariant::Light, retained_light_flavor.clone());
    let dark_options = variant_options(ThemeVariant::Dark, retained_dark_flavor.clone());

    let flavor_options_scroller = ScrolledWindow::builder().min_content_height(64).build();

    if toggled.get_blocking() {
        flavor_options_scroller.set_child(Some(&dark_options));
    } else {
        flavor_options_scroller.set_child(Some(&light_options));
    }

    toggled.connect(clone!(
        #[weak]
        flavor_options_scroller,
        #[strong]
        light_options,
        #[strong]
        dark_options,
        #[upgrade_or_default]
        move |_, toggled: bool| {
            if toggled {
                flavor_options_scroller.set_child(Some(&dark_options));
            } else {
                flavor_options_scroller.set_child(Some(&light_options));
            }

            Default::default()
        }
    ));

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
