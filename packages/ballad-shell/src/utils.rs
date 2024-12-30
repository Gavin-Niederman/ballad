use gtk::{Widget, prelude::*};

pub fn set_class_on_widget(enabled: bool, widget: &impl IsA<Widget>, class: &str) {
    if enabled {
        widget.add_css_class(class);
    } else {
        widget.remove_css_class(class);
    }
}

pub fn toggle_class_on_widget(widget: &impl IsA<Widget>, class: &str) {
    set_class_on_widget(widget.has_css_class(class), widget, class);
}
