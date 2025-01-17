use ballad_services::variable::Variable;
use gtk::{
    Box, Button, Image, Orientation, Revealer, Widget,
    glib::{self, clone},
    prelude::*,
};
use typed_builder::TypedBuilder;

use crate::utils::set_class_on_widget;

#[derive(TypedBuilder)]
#[builder(build_method(into = gtk::Box))]
pub struct DropdownButton<B: IsA<Widget>, R: IsA<Widget>, O: Fn(bool) + 'static> {
    pub button_content: B,
    pub dropdown_content: R,
    pub toggled: Variable,
    #[builder(setter(strip_option), default)]
    pub on_toggle: Option<O>,
}
impl<B: IsA<Widget>, R: IsA<Widget>, O: Fn(bool) + 'static> From<DropdownButton<B, R, O>>
    for gtk::Box
{
    fn from(props: DropdownButton<B, R, O>) -> Self {
        dropdown_button(props)
    }
}

pub fn dropdown_button<B: IsA<Widget>, R: IsA<Widget>, O: Fn(bool) + 'static>(
    DropdownButton {
        button_content,
        dropdown_content,
        toggled,
        on_toggle,
    }: DropdownButton<B, R, O>,
) -> gtk::Box {
    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .name("dropdown-button")
        .css_classes(["dropdown-button"])
        .build();

    toggled.connect_value_changed_typed(
        false,
        clone!(
            #[weak]
            container,
            move |_, toggled: bool| set_class_on_widget(toggled, &container, "toggled")
        ),
    );
    if toggled.value_typed().unwrap_or(false) {
        container.add_css_class("toggled");
    }

    let revealer = Revealer::new();
    let revealer_content_container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .name("dropdown-button-content")
        .css_classes(["revealer-content-container"])
        .build();

    revealer_content_container.append(&dropdown_content);

    let toggle_button_container = Box::builder()
        .name("toggle-container")
        .css_classes(["toggle-container"])
        .orientation(Orientation::Horizontal)
        .build();
    let toggle_button = Button::builder()
        .name("toggle-button")
        .css_classes(["toggle-button-toggle", "toggle-button-child"])
        .child(&button_content)
        .hexpand(true)
        .build();

    let caret_icon = Image::builder()
        .icon_name("caret-right-symbolic")
        .pixel_size(24)
        .build();
    let dropdown_button = Button::builder()
        .name("dropdown-button")
        .css_classes(["toggle-button-dropdown", "toggle-button-child"])
        .child(&caret_icon)
        .build();

    dropdown_button.connect_clicked(clone!(
        #[weak]
        revealer,
        #[weak]
        container,
        move |_| {
            let revealed = revealer.reveals_child();
            revealer.set_reveal_child(!revealed);
            set_class_on_widget(!revealed, &container, "dropped");
        }
    ));
    toggle_button.connect_clicked(clone!(
        #[weak]
        toggled,
        move |_| {
            toggled.set_value_typed(!toggled.value_typed::<bool>().unwrap());
            if let Some(on_toggle) = on_toggle.as_ref() {
                on_toggle(toggled.value_typed().unwrap_or(false));
            }
        }
    ));

    toggle_button_container.append(&toggle_button);
    toggle_button_container.append(&dropdown_button);

    revealer.connect_reveal_child_notify(clone!(
        #[weak]
        container,
        move |revealer| set_class_on_widget(revealer.reveals_child(), &container, "revealed")
    ));

    revealer.set_child(Some(&revealer_content_container));
    revealer.set_reveal_child(false);

    container.append(&toggle_button_container);
    container.append(&revealer);

    container
}
