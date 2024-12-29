mod widgets;

use gtk::{
    Application,
    gdk::{self, Monitor},
    glib::closure_local,
    prelude::*,
};
use gtk::{gio, glib};
use widgets::sidebar::{SideBarProperties, sidebar};

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

    let sidebars = monitors.map(|monitor| {
        sidebar(
            SideBarProperties::builder()
                .application(app)
                .monitor(monitor)
                .build(),
        )
    });

    sidebars.for_each(|sidebar| {
        sidebar.present();
    });
}

fn startup(_app: &Application) {
    ballad_services::config::CONFIG_SERVICE.with(|config_service| {
        config_service.connect_closure(
            "config-changed",
            false,
            closure_local!(move |_: ballad_services::config::ConfigService,
                                 config: &ballad_config::ShellConfig| {
                println!("Config changed: {:?}", config);
            }),
        );
    });
    ballad_services::battery::BATTERY_SERVICE.with(|battery_service| {
        battery_service.connect_charging_notify(|service| {
            println!("Battery percentage: {}", service.percentage());
        });
    });
}
