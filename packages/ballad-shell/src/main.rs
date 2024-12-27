mod widgets;

use gtk::{
    Application, IconTheme,
    gdk::{self, Monitor},
    prelude::*,
};
use widgets::sidebar::{SideBarProperties, sidebar};

fn main() {
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
    IconTheme::default().add_search_path("assets/icons");
}
