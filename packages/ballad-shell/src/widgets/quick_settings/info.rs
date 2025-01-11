use ballad_services::accounts::{ACCOUNTS_SERVICE, User};
use gtk::{prelude::BoxExt, Align, Image, Label, Orientation};

use crate::widgets::clock::{date, time};

fn current_user() -> User {
    let uid = unsafe { libc::getuid() };
    ACCOUNTS_SERVICE.with(|service| smol::block_on(service.find_user_by_id(uid as u64)).unwrap())
}

pub fn user_icon(size: i32) -> Image {
    let fallback_icon = Image::builder()
        .icon_name("avatar-default-symbolic")
        .pixel_size(size)
        .build();

    if let Some(icon_path) = current_user().icon_file() {
        let image = Image::from_file(&icon_path);
        image.set_pixel_size(size);
        return image;
    }

    fallback_icon
}

pub fn info_block() -> gtk::Box {
    let container = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .css_classes(["info-block"])
        .name("info-block")
        .spacing(12)
        .valign(Align::Center)
        .build();

    container.append(&user_icon(48));

    let right_widgets = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .hexpand(true)
        .name("right-widgets")
        .halign(Align::Start)
        .valign(Align::Start)
        .build();

    let username_text = current_user()
        .real_name()
        .unwrap_or_else(|| current_user().user_name());
    right_widgets.append(
        &Label::builder()
            .name("username")
            .css_classes(["username"])
            .hexpand(true)
            .halign(Align::Start)
            .label(username_text)
            .build(),
    );

    let time_widgets = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .name("time-widgets")
        .spacing(8)
        .build();
    time_widgets.append(&time());
    time_widgets.append(&date());
    right_widgets.append(&time_widgets);

    container.append(&right_widgets);

    container
}
