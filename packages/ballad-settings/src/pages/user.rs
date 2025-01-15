use ballad_services::accounts::ACCOUNTS_SERVICE;
use gtk::{Button, Entry, FileDialog, FileFilter, Window, gio::Cancellable, prelude::*};

use super::{Page, option};

pub fn user_page() -> gtk::Box {
    let icon_file_open_button = Button::builder()
        .label("Open File")
        .name("icon-file-open-button")
        .css_classes(["icon-file-open-button"])
        .build();
    icon_file_open_button.connect_clicked(|_| {
        let filter = FileFilter::new();
        filter.add_mime_type("image/*");

        let dialog = FileDialog::builder().default_filter(&filter).build();
        dialog.open(None::<&Window>, Cancellable::NONE, |response| {
            gtk::glib::spawn_future_local(async move {
                if let Ok(file) = response {
                    if let Some(user) =
                        ACCOUNTS_SERVICE.with(|service| smol::block_on(service.current_user()))
                    {
                        _ = user.set_icon(file.path().unwrap()).await;
                    }
                }
            });
        });
    });

    let name_input = Entry::builder()
        .visible(true)
        .name("name-input")
        .placeholder_text("Real Name")
        .build();
    name_input.connect_activate(|entry| {
        if let Some(user) = ACCOUNTS_SERVICE.with(|service| smol::block_on(service.current_user()))
        {
            _ = smol::block_on(user.set_real_name(entry.text().as_str()));
            entry.set_text("");
        }
    });

    let email_input = Entry::builder()
        .visible(true)
        .name("email-input")
        .placeholder_text("j.doe@example.com")
        .build();
    email_input.connect_activate(|entry| {
        if let Some(user) = ACCOUNTS_SERVICE.with(|service| smol::block_on(service.current_user()))
        {
            _ = smol::block_on(user.set_email(entry.text().as_str()));
            entry.set_text("");
        }
    });

    Page::builder()
        .name("user-page")
        .with_option(&option("Email", Some("The user's email"), &email_input))
        .with_option(&option(
            "Name",
            Some("The user's \"real name\" that appears in place of the username"),
            &name_input,
        ))
        .with_option(&option(
            "Icon",
            Some("The user's icon image"),
            &icon_file_open_button,
        ))
        .build()
}
