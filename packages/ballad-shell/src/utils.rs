use gtk::{Widget, prelude::*};

pub fn set_class_on_widget(enabled: bool, widget: &impl IsA<Widget>, class: &str) {
    if enabled {
        widget.add_css_class(class);
    } else {
        widget.remove_css_class(class);
    }
}
