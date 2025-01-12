use gtk::{prelude::*, Align, Label, Widget};

pub mod user;

pub fn settings_stack() -> gtk::Stack {
    let stack = gtk::Stack::builder().name("settings-stack").build();

    stack.add_titled(&user::user_page(), Some("user"), "User");

    stack
}

fn option<O: IsA<Widget>>(named: &str, option: &O) -> gtk::Box {
    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .css_classes(["option"])
        .name(named)
        .build();

    container.append(&Label::builder().label(named).halign(Align::Start).build());
    option.set_halign(Align::End);
    container.append(option);

    container
}
