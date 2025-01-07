use gtk::Button;
use gtk::Image;
use gtk::Label;
use gtk::Stack;
use gtk::glib;
use gtk::prelude::*;

pub fn user_select(stack: Stack) -> gtk::Box {
    let users = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(16)
        .name("user-select")
        .build();

    let add_user_graphics = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(8)
        .build();
    add_user_graphics.append(
        &Image::builder()
            .icon_name("new-user-symbolic")
            .pixel_size(24)
            .build(),
    );
    add_user_graphics.append(&Label::builder().label("Add User").build());

    let add_user_option = gtk::Button::builder()
        .css_classes(["user-select-option", "add-user"])
        .child(&add_user_graphics)
        .build();

    users.append(&add_user_option);

    users
}
