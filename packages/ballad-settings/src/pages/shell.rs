use std::{cell::LazyCell, collections::HashMap};

use super::{Page, option};
use ballad_config::{ShellConfig, ThemeConfig, theme::get_or_init_all_theme_selections};
use ballad_services::config::CONFIG_SERVICE;
use gtk::DropDown;

pub fn shell_page() -> gtk::Box {
    let theme_options = get_or_init_all_theme_selections();

    let theme_option_strings = theme_options
        .iter()
        .map(|option| option.theme().unwrap().name.clone())
        .collect::<Vec<_>>();
    let theme_selector = DropDown::from_strings(
        &theme_option_strings
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>(),
    );

    CONFIG_SERVICE.with(|service| {
        let service = LazyCell::force(service).clone();
        let theme = service.shell_config().theme.selected_theme;
        theme_selector.set_selected(theme_options.iter().position(|v| *v == theme).unwrap() as u32);

        theme_selector.connect_selected_notify(move |combo| {
            let theme = theme_options.get(combo.selected() as usize).unwrap();
            let config = service.shell_config();
            service.set_shell_config(ShellConfig {
                theme: ThemeConfig {
                    selected_theme: theme.clone(),
                    ..config.theme
                },
                ..config
            });
        })
    });

    Page::builder()
        .name("shell-page")
        .with_option(&option(
            "Theme",
            Some("The color scheme Ballad applications and the system will use"),
            &theme_selector,
        ))
        .build()
}
