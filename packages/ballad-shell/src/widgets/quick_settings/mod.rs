pub mod config;
pub mod dropdown_button;

use super::window::{Layer, LayershellWindow};
use config::{dark_theme_toggle_variable, on_theme_button_press};
use dropdown_button::DropdownButton;
use gtk::{
    Align, ApplicationWindow, Box, EventControllerKey, GestureClick, Image, Label, Orientation,
    Overlay,
    gdk::Key,
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

    let toggled = dark_theme_toggle_variable();

    let flavor_button_content = Box::builder()
        .orientation(Orientation::Horizontal)
        .halign(Align::Start)
        .spacing(8)
        .build();
    flavor_button_content.append(
        &Image::builder()
            .icon_name("theme-symbolic")
            .pixel_size(24)
            .build(),
    );
    flavor_button_content.append(
        &Label::builder()
            .label("Dark Mode")
            .vexpand(true)
            .valign(Align::Center)
            .build(),
    );
    let flavor_selector = DropdownButton::builder()
        .on_toggle(on_theme_button_press())
        .toggled(toggled)
        .button_content(flavor_button_content)
        .dropdown_content(Label::builder().label("DropC").build())
        .build();

    quick_settings.append(&flavor_selector);

    overlay.set_child(Some(&click_screen));
    overlay.add_overlay(&quick_settings);

    window.set_child(Some(&overlay));

    window
}
