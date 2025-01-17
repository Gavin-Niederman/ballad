mod brightness;
mod config;
mod dropdown_button;
mod flavor;
mod info;

use super::volume::Volume;
use super::window::{Layer, LayershellWindow};
use flavor::flavor_selector;
use gtk::gdk::Key;
use gtk::glib;
use gtk::{
    ApplicationWindow, Box, EventControllerKey, GestureClick, Orientation, Overlay, glib::clone,
    prelude::*,
};
use gtk4_layer_shell::{KeyboardMode, LayerShell};
use info::info_block;
use typed_builder::TypedBuilder;

pub const QUICK_SETTINGS_WINDOW_TITLE: &str = "quick-settings";

#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq)]
#[builder(build_method(into = ApplicationWindow))]
pub struct QuickSettings<'a> {
    pub application: &'a gtk::Application,
}
impl From<QuickSettings<'_>> for ApplicationWindow {
    fn from(props: QuickSettings) -> Self {
        quick_settings(props)
    }
}

pub fn quick_settings(QuickSettings { application }: QuickSettings) -> ApplicationWindow {
    let window: ApplicationWindow = LayershellWindow::builder()
        .layer(Layer::Top)
        .application(application)
        .title(QUICK_SETTINGS_WINDOW_TITLE)
        .build();
    window.set_keyboard_mode(KeyboardMode::OnDemand);

    let kbd_exit = EventControllerKey::builder()
        .name("close-quick-settings")
        .build();
    kbd_exit.connect_key_pressed(clone!(
        #[weak]
        window,
        #[upgrade_or]
        glib::Propagation::Proceed,
        move |_, key, _, _| {
            if key == Key::Escape || key == Key::q {
                window.set_visible(false);
            }
            glib::Propagation::Stop
        }
    ));
    window.add_controller(kbd_exit);

    let overlay = Overlay::builder().name("padding-container").build();

    let click_screen = Box::builder()
        .name("click-screen")
        .hexpand(true)
        .vexpand(true)
        .focusable(false)
        .build();
    let click_exit = GestureClick::builder().name("close-quick-settings").build();
    click_exit.connect_pressed(clone!(
        #[weak]
        window,
        move |_, _, _, _| {
            window.set_visible(false);
        }
    ));
    click_screen.add_controller(click_exit);

    let quick_settings = Box::builder()
        .name("quick-settings")
        .orientation(Orientation::Vertical)
        .halign(gtk::Align::Start)
        .valign(gtk::Align::End)
        .css_classes(["quick-settings"])
        .build();

    quick_settings.append(&info_block());
    quick_settings.append(
        &Volume::builder()
            .orientation(super::Orientation::Horizontal)
            .draw_value(false)
            .build(),
    );
    quick_settings.append(&brightness::brightness());
    quick_settings.append(&flavor_selector());

    overlay.set_child(Some(&click_screen));
    overlay.add_overlay(&quick_settings);

    window.set_child(Some(&overlay));

    window
}
