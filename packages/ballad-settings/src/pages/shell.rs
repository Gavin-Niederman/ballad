use std::{cell::LazyCell, collections::HashMap};

use super::{Page, option};
use ballad_config::{CatppuccinFlavor, ShellConfig, ThemeConfig};
use ballad_services::config::CONFIG_SERVICE;
use gtk::{DropDown, ListItemFactory, StringList, prelude::*};

pub fn shell_page() -> gtk::Box {
    let theme_options: HashMap<_, _> = [
        ("Frappé", CatppuccinFlavor::Frappe),
        ("Macchiato", CatppuccinFlavor::Macchiato),
        ("Mocha", CatppuccinFlavor::Mocha),
        ("Latte", CatppuccinFlavor::Latte),
    ]
    .into_iter()
    .collect();
    let theme_selector = DropDown::from_strings(&theme_options.keys().copied().collect::<Vec<_>>());

    CONFIG_SERVICE.with(|service| {
        let service = LazyCell::force(service).clone();
        let theme = service.shell_config().theme.catppuccin_flavor;
        theme_selector
            .set_selected(theme_options.iter().position(|(_, v)| v == &theme).unwrap() as u32);

        theme_selector.connect_selected_notify(move |combo| {
            let theme = theme_options.keys().nth(combo.selected() as usize).unwrap();
            let theme = theme_options.get(theme).unwrap();
            service.set_shell_config(ShellConfig {
                theme: ThemeConfig {
                    catppuccin_flavor: *theme,
                },
                ..service.shell_config()
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
