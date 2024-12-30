mod style;
mod widgets;

use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    sync::{LazyLock, RwLock},
};

use ballad_config::CatppuccinFlavor;
use gtk::{
    Application, CssProvider,
    gdk::{self, Display, Monitor},
    glib::closure_local,
    prelude::*,
    style_context_add_provider_for_display, style_context_remove_provider_for_display,
};
use gtk::{gio, glib};
use widgets::{
    PerMonitorWidget,
    clock_underlay::clock_underlay,
    quick_settings::QuickSettings,
    sidebar::{screen_bevels::screen_bevels, sidebar},
};

static WINDOW_IDS: LazyLock<RwLock<HashMap<String, u32>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

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

    fn log_window(window: &gtk::ApplicationWindow) {
        if let Some(title) = window.title() {
            WINDOW_IDS
                .write()
                .unwrap()
                .insert(title.to_string(), window.id());
        }
    }

    monitors.for_each(|monitor| {
        let properties: PerMonitorWidget = PerMonitorWidget::builder()
            .application(app)
            .monitor(monitor.clone())
            .build();

        let sidebar = sidebar(properties.clone());
        log_window(&sidebar);
        sidebar.present();

        let screen_bevels = screen_bevels(properties.clone());
        log_window(&screen_bevels);
        screen_bevels.present();

        let clock_underlay = clock_underlay(properties);
        log_window(&clock_underlay);
        clock_underlay.present();
    });

    let quick_settings = QuickSettings::builder()
        .application(app)
        .visible(true)
        .build();
    log_window(&quick_settings);
    quick_settings.present();
}

fn startup(_app: &Application) {
    watch_theme_config();
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

    let theme_settings = gio::Settings::new("org.gnome.desktop.interface");

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

                    let color = if config.catppuccin_flavor.is_dark() {
                        "prefer-dark"
                    } else {
                        "prefer-light"
                    };
                    let _ = theme_settings.set_string("color-scheme", color);
                    let flavor_stringified = match config.catppuccin_flavor {
                        CatppuccinFlavor::Latte => "latte",
                        CatppuccinFlavor::Frappe => "frappe",
                        CatppuccinFlavor::Macchiato => "macchiato",
                        CatppuccinFlavor::Mocha => "mocha",
                    };
                    let _ = theme_settings.set_string("gtk-theme", "");
                    let _ = theme_settings.set_string(
                        "gtk-theme",
                        &format!("catppuccin-{flavor_stringified}-sky-standard"),
                    );
                }
            ),
        );
    });
}
