use std::{
    cell::{Cell, LazyCell},
    rc::Rc,
};

use ballad_config::{CatppuccinFlavor, ShellConfig, ThemeConfig};
use ballad_services::{
    config::CONFIG_SERVICE,
    variable::{Variable, VariableInner},
};
use gtk::{
    glib::{Variant, clone},
    prelude::ObjectExt,
};

pub fn on_theme_button_press() -> impl Fn(bool) + 'static {
    CONFIG_SERVICE.with(|config_service| {
        let config_service = LazyCell::force(config_service).clone();

        move |dark: bool| {
            let current_config = config_service.shell_config();

            let retained_dark_flavor = Rc::new(Cell::new(CatppuccinFlavor::default()));
            if !dark {
                let current_flavor = current_config.theme.catppuccin_flavor;
                retained_dark_flavor.set(current_flavor);

                config_service.set_shell_config(ShellConfig {
                    theme: ThemeConfig {
                        catppuccin_flavor: CatppuccinFlavor::Latte,
                    },
                    ..current_config
                });
            } else {
                config_service.set_shell_config(ShellConfig {
                    theme: ThemeConfig {
                        catppuccin_flavor: retained_dark_flavor.get(),
                    },
                    ..current_config
                });
            }
        }
    })
}

pub fn dark_theme_toggle_variable() -> Variable {
    let initial = CONFIG_SERVICE.with(|config_service| {
        config_service
            .shell_config()
            .theme
            .catppuccin_flavor
            .is_dark()
    });
    let variable = Variable::with_value(initial.into());

    CONFIG_SERVICE.with(clone!(
        #[strong]
        variable,
        move |config_service| {
            let config_service = LazyCell::force(config_service).clone();

            config_service
                .bind_property(
                    "shell-config",
                    Box::leak(Box::new(variable.clone())),
                    "value",
                )
                .transform_to(|_, config: ShellConfig| {
                    Some(VariableInner::from(Variant::from(
                        config.theme.catppuccin_flavor.is_dark(),
                    )))
                })
                .build();
        }
    ));

    variable
}
