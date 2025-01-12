use ballad_services::accounts::{ACCOUNTS_SERVICE, User};
use gtk::glib;
use gtk::{Align, Image, Label, Orientation, Overflow, glib::clone, prelude::BoxExt};

use crate::widgets::clock::{date, time};

fn current_user() -> User {
    let uid = unsafe { libc::getuid() };
    ACCOUNTS_SERVICE.with(|service| smol::block_on(service.find_user_by_id(uid as u64)).unwrap())
}

pub fn user_icon(size: i32, user: User) -> Image {
    let icon = Image::builder()
        .icon_name("avatar-default-symbolic")
        .pixel_size(size)
        .build();

    if let Some(icon_path) = current_user().icon_file() {
        icon.set_from_file(Some(&icon_path));
    }

    user.connect_icon_file_notify(clone!(
        #[weak]
        icon,
        #[strong]
        user,
        move |_| {
            if let Some(icon_path) = user.icon_file() {
                icon.set_from_file(Some(&icon_path));
            }
        }
    ));

    icon
}

pub fn info_block() -> gtk::Box {
    let user = current_user();

    let container = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .css_classes(["info-block"])
        .name("info-block")
        .spacing(12)
        .valign(Align::Center)
        .build();

    let icon = gtk::Box::builder()
        .css_classes(["user-icon"])
        .name("user-icon")
        .overflow(Overflow::Hidden)
        .build();
    icon.append(&user_icon(48, user.clone()));
    container.append(&icon);

    let right_widgets = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .hexpand(true)
        .name("right-widgets")
        .halign(Align::Start)
        .valign(Align::Start)
        .build();

    let username_text = user
        .real_name()
        .unwrap_or_else(|| current_user().user_name());
    let username = Label::builder()
        .name("username")
        .css_classes(["username"])
        .hexpand(true)
        .halign(Align::Start)
        .label(username_text)
        .build();
    right_widgets.append(&username);

    user.connect_real_name_notify(clone!(
        #[weak]
        username,
        move |_| {
            username.set_label(
                &current_user()
                    .real_name()
                    .unwrap_or_else(|| current_user().user_name()),
            );
        }
    ));

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
