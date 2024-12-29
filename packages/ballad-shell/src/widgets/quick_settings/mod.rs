use super::window::{Layer, LayershellWindow};
use gtk::{
    ApplicationWindow, Box, GestureClick, Orientation, Overlay,
    glib::{self, clone},
    prelude::*,
};
use gtk4_layer_shell::{KeyboardMode, LayerShell};
use typed_builder::TypedBuilder;

pub const QUICK_SETTINGS_WINDOW_TITLE: &str = "quick-settings";

#[derive(Debug, Clone, TypedBuilder, PartialEq, Eq)]
#[builder(build_method(into = ApplicationWindow))]
pub struct QuickSettings<'a> {
    #[builder(default = false)]
    pub visible: bool,
    pub application: &'a gtk::Application,
}
impl From<QuickSettings<'_>> for ApplicationWindow {
    fn from(props: QuickSettings) -> Self {
        quick_settings(props)
    }
}

pub fn quick_settings(
    QuickSettings {
        visible,
        application,
    }: QuickSettings,
) -> ApplicationWindow {
    let window: ApplicationWindow = LayershellWindow::builder()
        .layer(Layer::Top)
        .application(application)
        .title(QUICK_SETTINGS_WINDOW_TITLE)
        .build();
    window.set_keyboard_mode(KeyboardMode::OnDemand);
    window.set_visible(visible);

    let overlay = Overlay::builder().name("padding-container").build();

    let click_screen = Box::builder()
        .name("click-screen")
        .hexpand(true)
        .vexpand(true)
        .focusable(false)
        .build();
    let controller = GestureClick::builder().name("close-quick-settings").build();
    controller.connect_pressed(clone!(
        #[weak]
        window,
        move |_, _, _, _| {
            window.set_visible(false);
        }
    ));
    click_screen.add_controller(controller);

    let quick_settings = Box::builder()
        .name("quick-settings")
        .orientation(Orientation::Vertical)
        .halign(gtk::Align::Start)
        .valign(gtk::Align::End)
        .css_classes(["quick-settings"])
        .build();

    overlay.set_child(Some(&click_screen));
    overlay.add_overlay(&quick_settings);

    window.set_child(Some(&overlay));

    window
}
