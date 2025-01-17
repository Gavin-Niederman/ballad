use std::cell::LazyCell;

use ballad_config::{ShellConfig, ThemeConfig};
use ballad_services::{
    config::CONFIG_SERVICE,
    variable::{Variable, VariableInner},
};
use gtk::{
    glib::{Variant, clone},
    prelude::ObjectExt,
};

pub fn on_theme_button_press(retained_light_flavor: Variable, retained_dark_flavor: Variable) -> impl Fn(bool) + 'static {
    CONFIG_SERVICE.with(|config_service| {
        let config_service = LazyCell::force(config_service).clone();

        move |dark: bool| {
            let current_config = config_service.shell_config();

            if !dark {
                config_service.set_shell_config(ShellConfig {
                    theme: ThemeConfig {
                        selected_theme: retained_light_flavor.value_typed().unwrap(),
                        ..current_config.theme
                    },
                    ..current_config
                });
            } else {
                config_service.set_shell_config(ShellConfig {
                    theme: ThemeConfig {
                        selected_theme: retained_dark_flavor.value_typed().unwrap(),
                        ..current_config.theme
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
            .selected_theme
            .is_dark()
            .unwrap_or_default()
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
                        config.theme.selected_theme.is_dark().unwrap(),
                    )))
                })
                .build();
        }
    ));

    variable
}
