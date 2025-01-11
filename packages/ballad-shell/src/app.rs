use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::OnceLock};

use crate::widgets::{
    PerMonitorWidget,
    clock_underlay::clock_underlay,
    quick_settings::QuickSettings,
    sidebar::{screen_bevels::screen_bevels, sidebar},
};
use ballad_config::CatppuccinFlavor;
use gtk::{
    Application, ApplicationWindow, CssProvider, Window,
    gdk::{self, Display, Monitor},
    glib::{ExitCode, closure_local},
    prelude::*,
    style_context_add_provider_for_display, style_context_remove_provider_for_display,
};
use gtk::{gio, glib};
use smol::channel::{self, Sender};

pub struct App {
    pub app: Application,
    pub window_ids: HashMap<String, u32>,
}
impl App {
    pub fn new(app: Application) -> Self {
        let (control_sender, control_receiver) = channel::bounded(1);
        _ = APP_CONTROL_SENDER.set(Sender::clone(&control_sender));

        gtk::glib::spawn_future_local(async move {
            while let Ok(control) = control_receiver.recv().await {
                APP.with(|this| {
                    if let Some(this) = this.borrow().as_ref() {
                        match control {
                            AppControl::Quit => this.quit(),
                            AppControl::ToggleWindow(title) => {
                                if let Some(window) = this.window_by_title(&title) {
                                    window.set_visible(!window.is_visible());
                                }
                            }
                        }
                    }
                });
            }
        });

        Self {
            app,
            window_ids: HashMap::new(),
        }
    }

    fn push_window(&mut self, window: &ApplicationWindow) {
        if let Some(title) = window.title() {
            self.window_ids.insert(title.to_string(), window.id());
        }
    }

    pub fn window_by_title(&self, title: &str) -> Option<Window> {
        let id = self.window_ids.get(title).copied();
        id.and_then(|id| self.window_by_id(id))
    }
    pub fn window_by_id(&self, id: u32) -> Option<Window> {
        self.app.window_by_id(id)
    }

    pub fn quit(&self) {
        self.app.quit();
    }
}

pub enum AppControl {
    Quit,
    ToggleWindow(String),
}

thread_local! {
    pub static APP: RefCell<Option<App>> = const { RefCell::new(None) };
}
pub static APP_CONTROL_SENDER: OnceLock<Sender<AppControl>> = const { OnceLock::new() };

pub fn launch_app(gtk_args: Vec<String>) -> Result<ExitCode, crate::Error> {
    gio::resources_register_include!("icons.gresource").unwrap();

    let application = Application::builder()
        .application_id("com.gavinniederman.ballad-shell")
        .build();

    application.connect_activate(activate);
    application.connect_startup(startup);

    APP.with(|app| {
        app.replace(Some(App::new(application.clone())));
    });

    // The regular `Application::run` function takes all the arguments from the command line including ones we parse ourselves.
    // Build a list of after args and binary name to get around this.
    let mut args = vec![std::env::args().next().unwrap()];
    args.extend(gtk_args);
    Ok(application.run_with_args(&args))
}

/// Gets the monitors for the default GDK display.
fn get_monitors() -> impl Iterator<Item = Monitor> {
    let display = gdk::Display::default().unwrap();
    let monitors = display.monitors();
    let monitors = monitors
        .iter()
        .map(|item| item.unwrap())
        .collect::<Vec<_>>();
    monitors.into_iter()
}

/// Constructs the shell UI.
fn activate(app: &Application) {
    let monitors = get_monitors();

    /// Adds the window ID to the global map.
    /// Used to toggle windows on and off.
    fn push_window_id(window: &gtk::ApplicationWindow) {
        APP.with(|app| {
            let mut app = app.borrow_mut();
            app.as_mut().unwrap().push_window(window);
        });
    }

    // Widgets that should appear on every monitor.
    monitors.for_each(|monitor| {
        // Create only one instance of the properties for each monitor because the config is the same for each widget.
        let properties: PerMonitorWidget = PerMonitorWidget::builder()
            .application(app)
            .monitor(monitor.clone())
            .build();

        let sidebar = sidebar(properties.clone());
        push_window_id(&sidebar);
        sidebar.present();

        let screen_bevels = screen_bevels(properties.clone());
        push_window_id(&screen_bevels);
        screen_bevels.present();

        let clock_underlay = clock_underlay(properties);
        push_window_id(&clock_underlay);
        clock_underlay.present();
    });

    let quick_settings = QuickSettings::builder().application(app).build();
    push_window_id(&quick_settings);
    quick_settings.present();
    quick_settings.set_visible(false);
}

fn startup(_app: &Application) {
    watch_theme_config();
}

fn watch_theme_config() {
    let config = ballad_config::get_or_init_shell_config();
    let initial_css = crate::style::compile_scss_for_flavor(config.theme.catppuccin_flavor);

    // The CSS provider is replaced when the theme changes.
    let provider = Rc::new(RefCell::new(CssProvider::new()));
    // Load the initial CSS.
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
                    let new_css = crate::style::compile_scss_for_flavor(config.catppuccin_flavor);
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

                    // Set the system GTK theme based on our theme config.
                    //TODO: With custom theme support this will need a complete rework.
                    //TODO: Maybe each theme should have an associated GTK theme?
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
