use ballad_services::accounts::ACCOUNTS_SERVICE;
use gtk::{
    gio::Cancellable, prelude::{BoxExt, ButtonExt, FileExt}, Button, FileDialog, FileFilter, Window
};

use super::option;

pub fn user_page() -> gtk::Box {
    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .css_classes(["page"])
        .name("user-page")
        .spacing(12)
        .build();

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
                    if let Some(user) = ACCOUNTS_SERVICE.with(|service| smol::block_on(service.current_user())) {
                        _ = user.set_icon(file.path().unwrap()).await;
                    }
                }
            });
        });
    });
    
    container.append(&option("Icon", &icon_file_open_button));

    container
}
