use gtk::{gdk::{self, Monitor}, prelude::*, Application};
use gtk4_layer_shell::{Edge, Layer, LayerShell};

fn main() {
    let app = Application::builder()
        .application_id("com.gavinniederman.ballad-shell")
        .build();

    app.connect_activate(activate);

    app.run();
}

fn get_monitors() -> impl Iterator<Item = Monitor> {
    let display = gdk::Display::default().unwrap();
    let monitors = display.monitors();
    let monitors = monitors.iter().map(|item| item.unwrap()).collect::<Vec<_>>();
    monitors.into_iter()
}

fn activate(app: &Application) {
    let monitors = get_monitors();

    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Ballad Shell")
        .build();

    window.init_layer_shell();
    window.set_layer(Layer::Top);
    window.set_title(Some("Ballad Shell"));
    window.set_anchor(Edge::Bottom, true);
    window.set_anchor(Edge::Top, true);
    window.set_anchor(Edge::Left, true);

    let button = gtk::Button::with_label("Hello, world!");
    window.set_child(Some(&button));

    window.present();
}