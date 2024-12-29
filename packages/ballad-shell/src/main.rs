mod style;
mod widgets;

use std::{cell::RefCell, rc::Rc};

use gtk::{
    Application, CssProvider,
    gdk::{self, Display, Monitor},
    glib::closure_local,
    prelude::*,
    style_context_add_provider_for_display, style_context_remove_provider_for_display,
};
use gtk::{gio, glib};
use widgets::sidebar::{screen_bevels::{screen_bevels, ScreenBevelsProperties}, sidebar, SideBarProperties};

fn main() {
    gio::resources_register_include!("icons.gresource").unwrap();

    let app = Application::builder()
        .application_id("com.gavinniederman.ballad-shell")
        .build();

    app.connect_activate(activate);
    app.connect_startup(startup);

    app.run();
}

fn get_monitors() -> impl Iterator<Item = Monitor> {
    let display = gdk::Display::default().unwrap();
    let monitors = display.monitors();
    let monitors = monitors
        .iter()
        .map(|item| item.unwrap())
        .collect::<Vec<_>>();
    monitors.into_iter()
}

fn activate(app: &Application) {
    let monitors = get_monitors();

    monitors.for_each(|monitor| {
        sidebar(
            SideBarProperties::builder()
                .application(app)
                .monitor(monitor.clone())
                .build(),
        ).present();
        screen_bevels(
            ScreenBevelsProperties::builder()
                .application(app)
                .monitor(monitor)
                .build(),
        ).present();
    });
}

fn startup(_app: &Application) {
    watch_theme_config();

    ballad_services::battery::BATTERY_SERVICE.with(|battery_service| {
        battery_service.connect_charging_notify(|service| {
            println!("Battery percentage: {}", service.percentage());
        });
    });
}

fn watch_theme_config() {
    let config = ballad_config::get_or_init_shell_config();
    let initial_css = style::compile_scss_for_flavor(config.theme.catppuccin_flavor);
    let provider = Rc::new(RefCell::new(CssProvider::new()));
    provider.borrow().load_from_string(&initial_css);
    style_context_add_provider_for_display(
        &Display::default().unwrap(),
        &*provider.borrow(),
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    ballad_services::config::CONFIG_SERVICE.with(|config_service| {
        config_service.connect_closure(
            "shell-theme-config-changed",
            false,
            closure_local!(
                #[strong]
                provider,
                move |_: ballad_services::config::ConfigService,
                      config: &ballad_config::ThemeConfig| {
                    let new_css = style::compile_scss_for_flavor(config.catppuccin_flavor);
                    let new_provider = CssProvider::new();
                    new_provider.load_from_string(&new_css);

                    style_context_remove_provider_for_display(
                        &Display::default().unwrap(),
                        &*provider.borrow(),
                    );
                    style_context_add_provider_for_display(
                        &Display::default().unwrap(),
                        &new_provider,
                        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
                    );

                    provider.replace(new_provider);
                }
            ),
        );
    });
}