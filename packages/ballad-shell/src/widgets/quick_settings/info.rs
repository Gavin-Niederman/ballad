use ballad_services::accounts::ACCOUNTS_SERVICE;
use gtk::{gdk::Texture, Image};

pub fn user_icon(size: i32) -> Image {
    let fallback_icon = Image::builder().icon_name("avatar-default-symbolic").pixel_size(size).build();

    let uid = unsafe { libc::getuid() };
    if let Some(icon_path) = ACCOUNTS_SERVICE.with(|service| {
        smol::block_on(service.find_user_by_id(uid as u64)).and_then(|user| user.icon_file())
    }) {
        let image =  Image::from_file(&icon_path);
        image.set_pixel_size(size);
        return image;
    }

    fallback_icon
}
